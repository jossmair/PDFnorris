//! Structured Word export. Paragraphs and table cells flow in the document;
//! illustrations keep their page position beside native text. No positioned
//! text boxes or whole-page screenshots of native text.
mod flow;
use flow::*;
mod fonts;
mod layout;
mod odt;
mod word;
use crate::document::{Editor, Result, guarded};

#[derive(Clone, Debug)]
struct Run {
    text: String,
    font: String,
    size: f64,
    color: [f64; 3],
    bold: bool,
    italic: bool,
    underline: bool,
    strike: bool,
    script: i8,
    // Source baseline in display coordinates (downwards); inferred rise is in
    // points upwards and preserves a PDF script's already reduced font size.
    baseline: Option<f64>,
    rise: Option<f64>,
    background: Option<[f64; 3]>,
    rect: Option<[f64; 4]>,
    fraction: Option<Box<Fraction>>,
}
#[derive(Clone, Debug)]
struct Fraction {
    numerator: Vec<Run>,
    denominator: Vec<Run>,
}
#[derive(Clone, Debug)]
struct Paragraph {
    rect: [f64; 4], // displayed page, top down, points
    runs: Vec<Run>,
    align: &'static str,
    spacing: Option<f64>,
}
#[derive(Clone, Debug)]
struct Picture {
    rect: [f64; 4],
    bytes: Vec<u8>,
}
#[derive(Clone, Debug)]
struct Cell {
    column: usize,
    span: usize,
    merge: Option<&'static str>,
    fill: Option<[f64; 3]>,
    paragraphs: Vec<Paragraph>,
}
#[derive(Clone, Debug)]
struct Table {
    rect: [f64; 4],
    xs: Vec<f64>,
    ys: Vec<f64>,
    rows: Vec<Vec<Cell>>,
    border: f64,
    color: [f64; 3],
}
#[derive(Clone, Debug)]
enum Item {
    Paragraph(Paragraph),
    Picture(Picture),
    Table(Table),
}
impl Item {
    fn rect(&self) -> [f64; 4] {
        match self {
            Self::Paragraph(p) => p.rect,
            Self::Picture(p) => p.rect,
            Self::Table(t) => t.rect,
        }
    }
}
#[derive(Debug)]
struct SourceFont {
    name: String,
    bytes: Vec<u8>,
    bold: bool,
    italic: bool,
}

#[derive(Debug)]
struct Page {
    width: f64,
    height: f64,
    margin: [f64; 4],
    items: Vec<Item>,
    decorations: Vec<Picture>,
    fonts: Vec<SourceFont>,
}
#[derive(Clone, Copy, PartialEq)]
pub enum Format {
    Docx,
    Odt,
}
impl Format {
    pub fn extension(self) -> &'static str {
        if self == Self::Docx { "docx" } else { "odt" }
    }
    pub fn label(self) -> &'static str {
        if self == Self::Docx {
            "Word"
        } else {
            "OpenDocument"
        }
    }
}

/// Converts a snapshot of the working PDF, including unsaved edits. Fails
/// without writing a partial file if a page cannot be read completely.
pub fn convert(bytes: Vec<u8>) -> Result<Vec<u8>> {
    convert_as(bytes, Format::Docx)
}
pub fn convert_as(bytes: Vec<u8>, format: Format) -> Result<Vec<u8>> {
    convert_with_progress(
        bytes,
        format,
        &std::sync::atomic::AtomicBool::new(false),
        |_, _| {},
    )
}
pub fn convert_with_progress(
    bytes: Vec<u8>,
    format: Format,
    cancelled: &std::sync::atomic::AtomicBool,
    progress: impl Fn(usize, usize),
) -> Result<Vec<u8>> {
    guarded(|| {
        let editor = Editor::from_bytes(bytes)?;
        if !editor.can_copy() {
            return Err("Ce PDF n’autorise pas l’exportation de son contenu.".into());
        }
        let mut pages = Vec::new();
        let check = || {
            if cancelled.load(std::sync::atomic::Ordering::Acquire) {
                Err("Export annulé.".to_string())
            } else {
                Ok(())
            }
        };
        progress(0, editor.info.pages.len());
        for index in 0..editor.info.pages.len() {
            check()?;
            pages.push(layout::page(&editor, index)?);
            progress(index + 1, editor.info.pages.len());
        }
        check()?;
        if format == Format::Docx {
            word::document(&pages)
        } else {
            odt::document(&pages)
        }
    })
}

#[cfg(test)]
mod tests;
