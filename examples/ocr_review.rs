//! Local OCR round-trip review. Input is read-only; every output goes to an explicit directory.
use pdfcraft_edit::{AddedText, Family};
use pdfcraft_render::{PageRenderer, RenderRequest, Tile};
use pdfnorris::{
    document::Editor,
    ocr::{self, ImageSource},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() < 2 {
        return Err("Usage: ocr_review INPUT.pdf OUTPUT_DIR [PAGE]".into());
    }
    let out = std::path::Path::new(&args[1]);
    std::fs::create_dir_all(out)?;
    let page = args
        .get(2)
        .and_then(|a| a.to_str())
        .and_then(|a| a.parse().ok())
        .unwrap_or(0);
    let mut e = Editor::open(std::path::Path::new(&args[0]))?;
    let p = e.info.pages[page].clone();
    let (bytes, area) = e.image_for_ocr(page, ImageSource::Original(0))?;
    let scale = 3_f32.min(4094. / (area[2] - area[0]).max(area[3] - area[1]));
    let x = (area[0] * scale).floor().max(0.) as u32;
    let y = (area[1] * scale).floor().max(0.) as u32;
    let mut renderer = PageRenderer::new(bytes, Default::default());
    let img = renderer.render(RenderRequest {
        page,
        scale,
        tile: Some(Tile {
            x,
            y,
            w: ((area[2] * scale).ceil() as u32 - x).max(1),
            h: ((area[3] * scale).ceil() as u32 - y).max(1),
        }),
        ..Default::default()
    });
    if let Some(error) = img.error {
        return Err(error.into());
    }
    std::fs::write(
        out.join("before.png"),
        ocr::encode_png(&img.rgba, img.width, img.height)?,
    )?;
    let mut result = ocr::recognize(&img.rgba, img.width, img.height)?;
    // Match the application: visible native corrections above the image must
    // not be duplicated by recognizing the older raster lettering underneath.
    let retained = e.image_ocr_retained_text(page, area)?;
    for line in &mut result.lines {
        line.words.retain(|word| {
            let r = [
                (x as f32 + word.rect[0]) / scale,
                (y as f32 + word.rect[1]) / scale,
                (x as f32 + word.rect[2]) / scale,
                (y as f32 + word.rect[3]) / scale,
            ];
            !retained.iter().any(|v| {
                let w = r[2].min(v[2] + 1.) - r[0].max(v[0] - 1.);
                let h = r[3].min(v[3] + 1.) - r[1].max(v[1] - 1.);
                w > 0. && h > 0. && w * h >= (r[2] - r[0]) * (r[3] - r[1]) * 0.25
            })
        });
        line.text = line
            .words
            .iter()
            .map(|w| w.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");
    }
    result.lines.retain(|l| !l.words.is_empty());
    let raw: Vec<_> = result.lines.iter().map(|line| serde_json::json!({
        "text": line.text, "rect": line.rect,
        "words": line.words.iter().map(|w| serde_json::json!({"text":w.text,"rect":w.rect})).collect::<Vec<_>>()
    })).collect();
    std::fs::write(
        out.join("recognition.json"),
        serde_json::to_vec_pretty(&raw)?,
    )?;
    let mut blocks = result.styled_blocks(&img.rgba, img.width, img.height);
    let clean = ocr::erase_letters(&img.rgba, img.width, img.height, &mut blocks)?;
    let png = ocr::encode_png(&clean, img.width, img.height)?;
    std::fs::write(out.join("background.png"), &png)?;
    let to_rect = |r: [f32; 4]| {
        [
            r[0] as f64,
            (p.height - r[3]) as f64,
            r[2] as f64,
            (p.height - r[1]) as f64,
        ]
    };
    let mut texts = Vec::new();
    let mut report = Vec::new();
    for b in blocks {
        let r = [
            area[0] + b.rect[0] / scale,
            area[1] + b.rect[1] / scale,
            area[0] + b.rect[2] / scale,
            area[1] + b.rect[3] / scale,
        ];
        let mut size = (b.size / scale as f64).clamp(3., 96.);
        let measured = Family::Helvetica
            .width(&b.text, size, b.bold, false)
            .max(1.);
        size = size
            .min(size * ((r[2] - r[0] + 3.) as f64) / measured)
            .max(3.);
        report.push(
            serde_json::json!({"text":b.text,"rect":r,"color":b.color,"size":size,"bold":b.bold}),
        );
        texts.push(AddedText {
            rect: to_rect([r[0], r[1], r[2] + 4., r[3] + 1.]),
            text: b.text,
            size,
            bold: b.bold,
            color: b.color,
            ..Default::default()
        });
    }
    std::fs::write(out.join("blocks.json"), serde_json::to_vec_pretty(&report)?)?;
    e.image_to_blocks(page, ImageSource::Original(0), &png, to_rect(area), texts)?;
    std::fs::write(out.join("editable.pdf"), &*e.bytes)?;
    let mut renderer = PageRenderer::new(e.bytes.clone(), Default::default());
    let after = renderer.render(RenderRequest {
        page,
        scale: 1.5,
        ..Default::default()
    });
    std::fs::write(
        out.join("after.png"),
        ocr::encode_png(&after.rgba, after.width, after.height)?,
    )?;
    println!("OCR round-trip reviewed; {} blocks.", report.len());
    Ok(())
}
