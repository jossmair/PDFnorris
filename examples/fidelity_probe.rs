//! Isolated native-edit probe. Its output contains geometry/status, never source text or names.
use pdfnorris::document::Editor;
use serde_json::{Value, json};
use std::path::Path;

fn exercise(source: &Path, output: &Path, operation: &str) -> Result<Value, String> {
    // An existing directory is refused, including the source's own directory.
    std::fs::create_dir(output).map_err(|e| e.to_string())?;
    let mut editor = Editor::open(source)?;
    let original = editor.bytes.clone();
    let mut candidate = None;
    for page in 0..editor.info.pages.len().min(3) {
        match editor.blocks(page) {
            Ok(blocks) => {
                if let Some((index, block)) = blocks
                    .iter()
                    .enumerate()
                    .find(|(_, b)| !b.invisible && !b.text.trim().is_empty())
                {
                    candidate = Some((page, index, block.rect));
                    break;
                }
            }
            Err(_) => {
                return Ok(
                    json!({"status":"refused_inventory", "unchanged":editor.bytes == original}),
                );
            }
        }
    }
    let Some((page, index, rect)) = candidate else {
        return Ok(json!({"status":"not_applicable", "reason":"no_native_editable_text"}));
    };
    let geometry = &editor.info.pages[page];
    let mut mask = [
        f32::INFINITY,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NEG_INFINITY,
    ];
    for (x, y) in [
        (rect[0], rect[1]),
        (rect[0], rect[3]),
        (rect[2], rect[1]),
        (rect[2], rect[3]),
    ] {
        for (dx, dy) in [(0., 0.), (3., 2.)] {
            let p = geometry.user_to_view((x + dx) as f32, (y + dy) as f32);
            mask = [
                mask[0].min(p[0]),
                mask[1].min(p[1]),
                mask[2].max(p[0]),
                mask[3].max(p[1]),
            ];
        }
    }
    let result = match operation {
        "move" => editor.move_block(page, index, [3., 2.]),
        "delete" => editor.delete_text_block(page, index),
        _ => return Err("Unknown operation".into()),
    };
    if result.is_err() {
        return Ok(
            json!({"status":"refused_edit", "unchanged":editor.bytes==original, "undo_unchanged":!editor.can_undo()}),
        );
    }
    let modified = editor.bytes.clone();
    editor.undo()?;
    if editor.bytes != original {
        return Err("undo_mismatch".into());
    }
    editor.redo()?;
    if editor.bytes != modified {
        return Err("redo_mismatch".into());
    }
    editor.save(&output.join("result.pdf"))?;
    drop(editor);
    let reopened = Editor::open(&output.join("result.pdf"))?;
    let extracted = reopened.text(page)?;
    Ok(
        json!({"status":"ok", "page":page, "mask_view":mask, "pages":reopened.info.pages.len(), "undo_exact":true, "redo_exact":true, "reopened":true, "characters_after":extracted.plain_text().chars().count()}),
    )
}

fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 3 {
        eprintln!("Usage: fidelity_probe source.pdf new-output-directory move|delete");
        std::process::exit(2);
    }
    let started = std::time::Instant::now();
    let result = pdfnorris::document::guarded(|| {
        exercise(
            Path::new(&args[0]),
            Path::new(&args[1]),
            &args[2].to_string_lossy(),
        )
    });
    let mut report = result.unwrap_or_else(|_| json!({"status":"error"}));
    report["seconds"] = json!(started.elapsed().as_secs_f64());
    println!("{report}");
}
