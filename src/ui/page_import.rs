use super::*;
use pdfnorris::document::PreparedPageImport;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc,
};

pub(super) struct Job {
    instance: u64,
    receiver: mpsc::Receiver<Result<PreparedPageImport>>,
    ready: Option<Result<PreparedPageImport>>,
    cancelled: Arc<AtomicBool>,
}
impl App {
    pub(super) fn start_page_import(&mut self, paths: &[PathBuf]) {
        if self.work.importing.is_some()
            || paths.is_empty()
            || self.workspace != Workspace::Pages
            || !self.finish_text()
            || !self.permit()
        {
            return;
        }
        let Some(editor) = &self.editor else { return };
        let request = match editor.page_import_request() {
            Ok(request) => request,
            Err(error) => {
                self.message = Some(error);
                return;
            }
        };
        let instance = editor.instance;
        let paths = paths.to_vec();
        let cancelled = Arc::new(AtomicBool::new(false));
        let worker_cancel = cancelled.clone();
        let (sender, receiver) = mpsc::channel();
        self.work.importing = Some(Job {
            instance,
            receiver,
            ready: None,
            cancelled,
        });
        self.message = None;
        std::thread::spawn(move || {
            let result = pdfnorris::document::guarded(|| {
                let mut sources = Vec::new();
                let mut total = 0usize;
                for path in paths {
                    if worker_cancel.load(Ordering::Acquire) {
                        return Err("Ajout annulé.".into());
                    }
                    let bytes = pdfnorris::files::read(&path)?;
                    total = total
                        .checked_add(bytes.len())
                        .ok_or("Sources trop volumineuses.")?;
                    if total > 256 * 1024 * 1024 {
                        return Err("L’ensemble des PDF dépasse 256 Mio ; ajoutez-les en plusieurs opérations.".into());
                    }
                    sources.push((
                        path.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned(),
                        bytes,
                    ));
                }
                request.prepare(sources, &worker_cancel)
            });
            let _ = sender.send(result);
        });
    }
    pub(super) fn page_import_target(&self) -> Option<usize> {
        let job = self.work.importing.as_ref()?;
        if job.ready.is_none()
            || self
                .editor
                .as_ref()
                .is_some_and(|e| e.instance == job.instance)
        {
            return None;
        }
        self.documents.iter().position(|s| {
            s.as_ref()
                .and_then(|s| s.editor())
                .is_some_and(|e| e.instance == job.instance)
        })
    }
    pub(super) fn cancel_page_import(&mut self) {
        if let Some(job) = &self.work.importing {
            job.cancelled.store(true, Ordering::Release);
        }
    }
    pub(super) fn poll_page_import(&mut self, ctx: &egui::Context) {
        let Some(mut job) = self.work.importing.take() else {
            return;
        };
        if job.ready.is_none() {
            match job.receiver.try_recv() {
                Ok(result) => job.ready = Some(result),
                Err(mpsc::TryRecvError::Disconnected) => {
                    job.ready = Some(Err(
                        "L’ajout de pages a été interrompu. Le document est conservé.".into(),
                    ))
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        if job.cancelled.load(Ordering::Acquire) {
            if job.ready.is_none() {
                self.work.importing = Some(job);
            }
            return;
        }
        let active = self
            .editor
            .as_ref()
            .is_some_and(|e| e.instance == job.instance);
        let exists = active
            || self
                .documents
                .iter()
                .flatten()
                .any(|s| s.editor().is_some_and(|e| e.instance == job.instance));
        if !exists {
            job.cancelled.store(true, Ordering::Release);
            return;
        }
        if active && job.ready.is_some() && self.finish_text() {
            if let Some(result) = job.ready.take() {
                let result = result.and_then(|prepared| {
                    self.editor
                        .as_mut()
                        .ok_or_else(|| "Le document a été fermé.".to_owned())?
                        .apply_page_import(prepared)
                });
                match result {
                    Ok(pages) => {
                        self.page = pages.start;
                        self.page_destination = self.page + 1;
                        self.reset_page_views(true);
                        self.page_selection.replace(pages);
                        self.jump = Some(self.page);
                        self.message = None;
                    }
                    Err(error) => self.message = Some(error),
                }
                ctx.request_repaint();
                return;
            }
        }
        self.work.importing = Some(job);
    }
}
