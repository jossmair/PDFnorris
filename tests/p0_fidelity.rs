use pdfcraft_cos::{Dict, Object, Stream};
use pdfnorris::document::Editor;

fn source(parts: &[&[u8]]) -> Editor {
    let mut doc = pdfcraft_create::blank(320., 240., 1).unwrap();
    let mut font = Dict::new();
    font.set(b"Subtype".to_vec(), Object::name("Type1"));
    font.set(b"BaseFont".to_vec(), Object::name("Helvetica"));
    let mut fonts = Dict::new();
    fonts.set(b"F1".to_vec(), Object::Dict(font));
    let mut res = Dict::new();
    res.set(b"Font".to_vec(), Object::Dict(fonts));
    let contents = parts
        .iter()
        .map(|p| Object::Ref(doc.add(Stream::flate(Dict::new(), p))))
        .collect();
    let page = pdfcraft_model::pages(&doc)[0].obj;
    doc.update_dict(page, |d| {
        d.set(b"Resources".to_vec(), Object::Dict(res));
        d.set(b"Contents".to_vec(), Object::Array(contents));
    })
    .unwrap();
    Editor::from_bytes(pdfcraft_cos::write_full(&doc, &Default::default()).unwrap()).unwrap()
}

#[test]
fn rewritten_stream_keeps_marks_and_drops_stale_decoded_length() {
    let content = b"BT /F1 14 Tf 20 170 Td (TARGET) Tj ET";
    let mut e = source(&[content]);
    let page = pdfcraft_model::pages(&e.doc)[0].obj;
    let mut dict = Dict::new();
    dict.set(b"DL".to_vec(), Object::Int(content.len() as i64));
    dict.set(b"PCMark".to_vec(), Object::name("fixture"));
    let stream = e.doc.add(Stream::flate(dict, content));
    e.doc
        .update_dict(page, |d| d.set(b"Contents".to_vec(), Object::Ref(stream)))
        .unwrap();
    let bytes = pdfcraft_cos::write_full(&e.doc, &Default::default()).unwrap();
    let mut e = Editor::from_bytes(bytes).unwrap();
    e.delete_text_block(0, 0).unwrap();
    let reopened = Editor::from_bytes(e.bytes.as_ref().clone()).unwrap();
    let page = &pdfcraft_model::pages(&reopened.doc)[0];
    let stream = reopened.doc.resolve(page.dict.get(b"Contents").unwrap());
    let Object::Stream(stream) = stream.as_ref() else {
        panic!("expected one stream")
    };
    assert_eq!(stream.dict.get(b"PCMark"), Some(&Object::name("fixture")));
    assert!(stream.dict.get(b"DL").is_none());
}

#[test]
fn private_paragraph_metadata_must_not_bypass_text_clip_protection() {
    for mode in 4..=7 {
        let body = format!(
            "/P << /PNText (untrusted) >> BDC q BT /F1 24 Tf {mode} Tr 20 170 Td (MASK) Tj ET 0 0 1 rg 0 0 320 240 re f Q EMC"
        );
        let mut e = source(&[body.as_bytes()]);
        assert_eq!(e.blocks(0).unwrap().len(), 1);
        let before = e.bytes.clone();
        assert!(
            e.delete_text_block(0, 0).is_err(),
            "Mode {mode}: deleting glyphs also removed the masked artwork"
        );
        assert_eq!(e.bytes, before);
        assert!(!e.can_undo());
        let mut clone = e.doc.clone();
        assert!(pdfcraft_edit::text::clear_blocks(&mut clone, 0, &[0]).is_err());
        assert_eq!(
            pdfcraft_cos::write_full(&clone, &Default::default()).unwrap(),
            pdfcraft_cos::write_full(&e.doc, &Default::default()).unwrap()
        );
    }
}

#[test]
fn undecodable_content_cannot_be_treated_as_an_empty_stream_during_editing() {
    let mut e = source(&[b"BT /F1 14 Tf 20 170 Td (TARGET) Tj ET"]);
    let mut bad = Dict::new();
    bad.set(b"Filter".to_vec(), Object::name("FlateDecode"));
    let broken = e.doc.add(Stream::from_raw(bad, vec![1, 2, 3, 4]));
    let page = pdfcraft_model::pages(&e.doc)[0].obj;
    e.doc
        .update_dict(page, |d| {
            let Object::Array(a) = d.get(b"Contents").unwrap().clone() else {
                panic!()
            };
            let mut all = a;
            all.push(Object::Ref(broken));
            d.set(b"Contents".to_vec(), Object::Array(all));
        })
        .unwrap();
    let mut e =
        Editor::from_bytes(pdfcraft_cos::write_full(&e.doc, &Default::default()).unwrap()).unwrap();
    let before = e.bytes.clone();
    assert!(
        e.delete_text_block(0, 0).is_err(),
        "A partially decoded page is not a safe editing inventory"
    );
    assert_eq!(e.bytes, before);
    assert!(!e.can_undo());
}
