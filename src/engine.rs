//! Adapter for revision I/O. Editing operations and rendering keep their own APIs.
use crate::document::Result;
use pdfcraft_cos::{Document, SaveOptions};
use pdfcraft_render::DocInfo;
use std::sync::Arc;

pub(crate) fn open(bytes: &Arc<Vec<u8>>) -> Result<Document> {
    Document::open(Arc::clone(bytes))
        .map_err(|e| format!("Ouverture impossible (document éventuellement protégé) : {e}"))
}

pub(crate) fn inspect(bytes: &Arc<Vec<u8>>) -> Result<DocInfo> {
    pdfcraft_render::inspect(Arc::clone(bytes), None).map_err(|e| e.to_string())
}

pub(crate) fn serialize(doc: &Document) -> Result<Arc<Vec<u8>>> {
    pdfcraft_cos::write_incremental(doc, &SaveOptions::default())
        .map(Arc::new)
        .map_err(|e| e.to_string())
}
