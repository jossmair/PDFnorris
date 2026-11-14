//! Compare sequential and batched paragraph clearing on the same synthetic input.
use pdfcraft_cos::{Dict, Object, Stream};
use std::time::Instant;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = pdfcraft_create::blank(300., 6000., 1)?;
    let mut font = Dict::new();
    font.set(b"Type".to_vec(), Object::name("Font"));
    font.set(b"Subtype".to_vec(), Object::name("Type1"));
    font.set(b"BaseFont".to_vec(), Object::name("Helvetica"));
    let mut fonts = Dict::new();
    fonts.set(b"F1".to_vec(), Object::Ref(doc.add(Object::Dict(font))));
    let mut resources = Dict::new();
    resources.set(b"Font".to_vec(), Object::Dict(fonts));
    let mut stream = String::from("10 10 20 20 re S\n");
    for i in 0..150 {
        let y = 40 + i * 36;
        stream.push_str(&format!(
            "BT /F1 12 Tf 40 {y} Td (Synthetic paragraph {i}) Tj ET\n"
        ));
    }
    let id = doc.add(Stream::flate(Dict::new(), stream.as_bytes()));
    let page = pdfcraft_model::pages(&doc)
        .first()
        .ok_or("Missing fixture page")?
        .obj;
    doc.update_dict(page, |d| {
        d.set(b"Resources".to_vec(), Object::Dict(resources));
        d.set(b"Contents".to_vec(), Object::Ref(id));
    })?;
    let count = pdfcraft_edit::text_blocks(&doc, 0)?.len();
    let selected: Vec<_> = (1..count).collect();
    let mut rows = vec![];
    for _ in 0..3 {
        let mut sequential = doc.clone();
        let start = Instant::now();
        for index in selected.iter().rev() {
            pdfcraft_edit::text::clear_block(&mut sequential, 0, *index)?;
        }
        let sequential_ms = start.elapsed().as_secs_f64() * 1000.;
        let mut batched = doc.clone();
        let start = Instant::now();
        pdfcraft_edit::text::clear_blocks(&mut batched, 0, &selected)?;
        let batched_ms = start.elapsed().as_secs_f64() * 1000.;
        let a = pdfcraft_edit::text_lines(&sequential, 0)?;
        let b = pdfcraft_edit::text_lines(&batched, 0)?;
        if a.len() != b.len()
            || a.iter().zip(&b).any(|(a, b)| {
                a.text != b.text
                    || a.rect
                        .iter()
                        .zip(b.rect)
                        .any(|(a, b)| (*a - b).abs() > 0.00001)
            })
        {
            return Err("Sequential and batched results differ".into());
        }
        rows.push(serde_json::json!({"sequential_ms":sequential_ms,"batched_ms":batched_ms,"same_remaining_text_and_geometry":true}));
    }
    println!(
        "{}",
        serde_json::json!({"paragraphs":count,"removed":selected.len(),"runs":rows})
    );
    Ok(())
}
