use pdfnorris::document::Editor;
use std::sync::atomic::AtomicBool;
fn pdf(pages: usize) -> Vec<u8> {
    let doc = pdfcraft_create::blank(240., 300., pages).unwrap();
    pdfcraft_cos::write_full(&doc, &Default::default()).unwrap()
}
#[test]
fn worker_import_commits_once_and_reopens_with_exact_undo_redo() {
    let mut editor = Editor::from_bytes(pdf(1)).unwrap();
    let before = editor.bytes.clone();
    let source = pdf(2);
    let original_source = source.clone();
    let request = editor.page_import_request().unwrap();
    let prepared = std::thread::spawn(move || {
        request.prepare(
            vec![("Synthetic.pdf".into(), source)],
            &AtomicBool::new(false),
        )
    })
    .join()
    .unwrap()
    .unwrap();
    assert_eq!(editor.bytes, before);
    assert_eq!(editor.apply_page_import(prepared).unwrap(), 1..3);
    let after = editor.bytes.clone();
    assert_eq!(
        Editor::from_bytes(after.as_ref().clone())
            .unwrap()
            .info
            .pages
            .len(),
        3
    );
    editor.undo().unwrap();
    assert_eq!(editor.bytes, before);
    assert!(!editor.can_undo());
    editor.redo().unwrap();
    assert_eq!(editor.bytes, after);
    assert_eq!(
        Editor::from_bytes(original_source)
            .unwrap()
            .info
            .pages
            .len(),
        2
    );
}
#[test]
fn cancellation_failures_and_stale_results_preserve_current_document() {
    let mut editor = Editor::from_bytes(pdf(1)).unwrap();
    let before = editor.bytes.clone();
    assert!(
        editor
            .page_import_request()
            .unwrap()
            .prepare(vec![("x.pdf".into(), pdf(1))], &AtomicBool::new(true))
            .is_err()
    );
    assert!(
        editor
            .page_import_request()
            .unwrap()
            .prepare(
                vec![("bad.pdf".into(), b"broken PDF".to_vec())],
                &AtomicBool::new(false)
            )
            .is_err()
    );
    assert_eq!(editor.bytes, before);
    assert!(!editor.can_undo());
    let prepared = editor
        .page_import_request()
        .unwrap()
        .prepare(vec![("x.pdf".into(), pdf(1))], &AtomicBool::new(false))
        .unwrap();
    editor.rotate_page(0, 90).unwrap();
    let changed = editor.bytes.clone();
    assert!(editor.apply_page_import(prepared).is_err());
    assert_eq!(editor.bytes, changed);
    editor.undo().unwrap();
    assert_eq!(editor.bytes, before);
}
