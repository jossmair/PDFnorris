#[test]
fn tall_figure_keeps_nested_text_and_small_picture_in_one_column() {
    use super::*;
    let picture = |rect| {
        Item::Picture(Picture {
            rect,
            bytes: vec![],
        })
    };
    let text = |rect| {
        Item::Paragraph(Paragraph {
            rect,
            runs: vec![],
            align: "left",
            spacing: None,
        })
    };
    let page = Page {
        width: 595.,
        height: 842.,
        margin: [24., 0., 10., 0.],
        items: vec![
            picture([360., 20., 580., 260.]),
            picture([220., 22., 340., 82.]),
            text([24., 25., 190., 39.]),
            text([24., 110., 340., 125.]),
        ],
        decorations: vec![],
        fonts: vec![],
    };
    let (end, cols) = columns(&page, 0);
    assert_eq!(end, 4);
    assert_eq!(cols.len(), 2);
    assert_eq!(cols[0].len(), 3);
    assert_eq!(cols[1].len(), 1);
    assert!(matches!(cols[1][0], Item::Picture(_)));
}
#[test]
#[ignore = "Private local diagnostic; requires PDFINATOR_CORPUS_PDF"]
fn inspect_private_export_layout() {
    use super::*;
    let path = std::env::var_os("PDFINATOR_CORPUS_PDF").unwrap();
    let editor = Editor::open(std::path::Path::new(&path)).unwrap();
    let mut text = String::new();
    for n in 0..editor.info.pages.len() {
        for paragraph in layout::paragraphs(&editor, n).unwrap() {
            text.push_str(&format!("SOURCE {paragraph:?}\n"));
        }
        let p = layout::page(&editor, n).unwrap();
        text.push_str(&format!(
            "PAGE {n} {} {} {:?}\n",
            p.width, p.height, p.margin
        ));
        for item in &p.items {
            match item {
                Item::Paragraph(p) => text.push_str(&format!(
                    "PAR {:?} {:?}\n",
                    p.rect,
                    p.runs
                        .iter()
                        .map(|r| (&r.text, r.size, r.rect, r.rise))
                        .collect::<Vec<_>>()
                )),
                Item::Picture(p) => text.push_str(&format!("IMAGE {:?}\n", p.rect)),
                Item::Table(t) => {
                    text.push_str(&format!("TABLE {:?} X {:?} Y {:?}\n", t.rect, t.xs, t.ys))
                }
            };
        }
    }
    std::fs::write(output("private-export-layout.txt"), text).unwrap();
}
#[test]
fn centered_column_titles_keep_wider_tables_in_their_columns() {
    use super::*;
    let title = |rect| {
        Item::Paragraph(Paragraph {
            rect,
            runs: vec![],
            align: "center",
            spacing: None,
        })
    };
    let table = |rect| {
        Item::Table(Table {
            rect,
            xs: vec![],
            ys: vec![],
            rows: vec![],
            border: 1.,
            color: [0.; 3],
        })
    };
    let page = Page {
        width: 720.,
        height: 540.,
        margin: [40., 0., 20., 0.],
        items: vec![
            title([124., 250., 284., 282.]),
            title([429., 250., 590., 282.]),
            table([88., 292., 320., 394.]),
            table([393., 292., 626., 394.]),
        ],
        decorations: vec![],
        fonts: vec![],
    };
    let (end, cols) = columns(&page, 0);
    assert_eq!(end, 4, "Both tables belong to the same row of columns");
    assert_eq!(cols.iter().map(Vec::len).collect::<Vec<_>>(), [2, 2]);
    assert!(matches!(cols[1][1], Item::Table(_)));
}
#[test]
fn white_slide_background_does_not_flatten_text_and_duplicate_tables() {
    use super::*;
    use pdfcraft_cos::{Dict, Object, Stream};
    let mut doc = pdfcraft_create::blank(720., 540., 1).unwrap();
    let mut face = Dict::new();
    face.set(b"Type".to_vec(), Object::name("Font"));
    face.set(b"Subtype".to_vec(), Object::name("Type1"));
    face.set(b"BaseFont".to_vec(), Object::name("Helvetica"));
    let mut fonts = Dict::new();
    fonts.set(b"F1".to_vec(), Object::Ref(doc.add(face)));
    let mut resources = Dict::new();
    resources.set(b"Font".to_vec(), Object::Dict(fonts));
    let stream = doc.add(Stream::flate(Dict::new(), b"1 1 1 rg 0 0 720 540 re f 0 0 0 rg 0 0 0 RG 1 w 100 200 200 80 re S 200 200 m 200 280 l S 100 240 m 300 240 l S 300 230 m 330 230 330 270 300 270 c S 99 157 29 16 re S BT /F1 12 Tf 100 160 Td (YES) Tj ET BT /F1 18 Tf 100 450 Td (Editable heading) Tj ET BT /F1 12 Tf 120 255 Td (First) Tj ET BT /F1 12 Tf 220 255 Td (Second) Tj ET BT /F1 12 Tf 120 215 Td (Third) Tj ET BT /F1 12 Tf 220 215 Td (Fourth) Tj ET"));
    let obj = pdfcraft_model::pages(&doc)[0].obj;
    doc.update_dict(obj, |d| {
        d.set(b"Resources".to_vec(), Object::Dict(resources));
        d.set(b"Contents".to_vec(), Object::Ref(stream));
    })
    .unwrap();
    let editor =
        Editor::from_bytes(pdfcraft_cos::write_full(&doc, &Default::default()).unwrap()).unwrap();
    let page = layout::page(&editor, 0).unwrap();
    assert!(page.items.iter().any(|i| matches!(i, Item::Table(_))));
    assert!(page.items.iter().any(|i| matches!(i, Item::Paragraph(p) if p.runs.iter().map(|r| r.text.as_str()).collect::<String>().contains("Editable heading"))));
    assert!(
        page.items.iter().any(
            |i| matches!(i, Item::Paragraph(p) if p.runs.iter().any(|r| r.text.contains("YES")))
        )
    );
    assert!(
        !page.decorations.is_empty(),
        "Table callout and text frame stay anchored"
    );
    assert!(
        !page.items.iter().any(|i| matches!(i, Item::Picture(_))),
        "Ornaments must not duplicate table/text in inline crops"
    );
    assert!(!page.items.iter().any(|i| matches!(i, Item::Picture(p) if (p.rect[2]-p.rect[0])*(p.rect[3]-p.rect[1]) > 720.*540.*0.9)));
}
#[test]
fn full_page_images_export_without_extra_page_carriers() {
    use super::*;
    let png = crate::ocr::encode_png(&[40, 120, 200, 255], 1, 1).unwrap();
    let pages: Vec<_> = (0..2)
        .map(|_| Page {
            width: 720.,
            height: 540.,
            margin: [0.; 4],
            items: vec![Item::Picture(Picture {
                rect: [0., 0., 720., 540.],
                bytes: png.clone(),
            })],
            decorations: Vec::new(),
            fonts: Vec::new(),
        })
        .collect();
    std::fs::write(
        output("corpus-full-page.docx"),
        word::document(&pages).unwrap(),
    )
    .unwrap();
    std::fs::write(
        output("corpus-full-page.odt"),
        odt::document(&pages).unwrap(),
    )
    .unwrap();
}
#[test]
fn separated_small_footer_stays_native_without_entering_body_flow() {
    let mut editor = Editor::from_bytes(
        pdfcraft_cos::write_full(
            &pdfcraft_create::blank(595., 842., 2).unwrap(),
            &Default::default(),
        )
        .unwrap(),
    )
    .unwrap();
    for page in 0..2 {
        for (text, rect, size) in [
            ("A real editable paragraph", [40., 700., 500., 730.], 12.),
            ("Example footer", [400., 20., 550., 30.], 8.),
        ] {
            editor
                .add_text(
                    page,
                    pdfcraft_edit::AddedText {
                        rect,
                        text: text.into(),
                        size,
                        ..Default::default()
                    },
                )
                .unwrap();
        }
        let mut p = layout::page(&editor, page).unwrap();
        let footer = p.items.iter().find(|i| is_footer(&p, i)).unwrap().clone();
        assert_eq!(p.items.iter().filter(|i| is_footer(&p, i)).count(), 1);
        assert_eq!(
            body_without_footers(&p).unwrap().items.len(),
            p.items.len() - 1
        );
        // A small line continuous with the body is not a footer.
        let mut preceding = footer.clone();
        if let Item::Paragraph(p) = &mut preceding {
            p.rect[1] -= 12.;
            p.rect[3] -= 12.;
        }
        p.items.push(preceding);
        assert!(!is_footer(&p, &footer));
    }
    std::fs::write(
        output("native-footer.docx"),
        convert(editor.bytes.as_ref().clone()).unwrap(),
    )
    .unwrap();
    std::fs::write(
        output("native-footer.odt"),
        convert_as(editor.bytes.as_ref().clone(), Format::Odt).unwrap(),
    )
    .unwrap();
}
#[test]
fn native_scripts_keep_their_source_size_and_baseline_offsets() {
    use pdfcraft_cos::{Dict, Object, Stream};
    let mut doc = pdfcraft_create::blank(595., 842., 1).unwrap();
    let mut face = Dict::new();
    face.set(b"Type".to_vec(), Object::name("Font"));
    face.set(b"Subtype".to_vec(), Object::name("Type1"));
    face.set(b"BaseFont".to_vec(), Object::name("Helvetica"));
    let mut fonts = Dict::new();
    fonts.set(b"F1".to_vec(), Object::Ref(doc.add(face)));
    let mut resources = Dict::new();
    resources.set(b"Font".to_vec(), Object::Dict(fonts));
    let stream = doc.add(Stream::flate(Dict::new(), b"BT /F1 14 Tf 50 700 Td (H) Tj /F1 9 Tf -4 Ts (2) Tj 0 Ts /F1 14 Tf (O) Tj ET BT /F1 14 Tf 50 640 Td (x) Tj /F1 9 Tf 6 Ts (2) Tj 0 Ts /F1 14 Tf (+y) Tj ET BT /F1 14 Tf 50 580 Td (A) Tj /F1 9 Tf (3) Tj /F1 14 Tf (B) Tj ET"));
    let page = pdfcraft_model::pages(&doc)[0].obj;
    doc.update_dict(page, |d| {
        d.set(b"Resources".to_vec(), Object::Dict(resources));
        d.set(b"Contents".to_vec(), Object::Ref(stream));
    })
    .unwrap();
    let editor =
        super::Editor::from_bytes(pdfcraft_cos::write_full(&doc, &Default::default()).unwrap())
            .unwrap();
    let page = super::layout::page(&editor, 0).unwrap();
    let scripts: Vec<_> = page
        .items
        .iter()
        .filter_map(|i| match i {
            super::Item::Paragraph(p) => Some(&p.runs),
            _ => None,
        })
        .flatten()
        .filter(|r| r.text.trim() == "2")
        .collect();
    assert_eq!(scripts.len(), 2);
    assert_eq!(scripts[0].script, -1, "The lowered 2 must stay an index");
    assert_eq!(scripts[1].script, 1, "The raised 2 must stay an exponent");
    assert!(scripts.iter().all(|r| (r.size - 9.).abs() < 0.01));
    assert!((scripts[0].rise.unwrap() + 4.).abs() < 0.01);
    assert!((scripts[1].rise.unwrap() - 6.).abs() < 0.01);
    let small = page
        .items
        .iter()
        .filter_map(|i| match i {
            super::Item::Paragraph(p) => Some(&p.runs),
            _ => None,
        })
        .flatten()
        .find(|r| r.text.trim() == "3")
        .unwrap();
    assert_eq!(
        small.script, 0,
        "A small font on the baseline is not an index"
    );
    assert!(small.rise.is_none());
    let word = super::word::run(scripts[0]);
    assert!(word.contains("<w:position w:val=\"-8\"/>"));
    assert!(
        !word.contains("vertAlign"),
        "Do not shrink a source-sized index a second time"
    );
}
#[test]
fn percentages_export_regression() {
    let Some(path) = std::env::var_os("PDFINATOR_PERCENTAGES_PDF") else {
        return;
    };
    let bytes = std::fs::read(path).unwrap();
    let editor = super::Editor::from_bytes(bytes).unwrap();
    std::fs::write(
        output("47-drawings.txt"),
        format!("{:#?}", editor.export_drawings(0).unwrap()),
    )
    .unwrap();
    let pages: Vec<_> = (0..editor.info.pages.len())
        .map(|i| super::layout::page(&editor, i).unwrap())
        .collect();
    let fractions: Vec<_> = pages
        .iter()
        .flat_map(|p| &p.items)
        .filter_map(|i| {
            if let super::Item::Paragraph(p) = i {
                Some(&p.runs)
            } else {
                None
            }
        })
        .flatten()
        .filter_map(|r| r.fraction.as_ref())
        .collect();
    assert_eq!(
        fractions.len(),
        5,
        "All five fractions must be editable, including the two grey filled bars"
    );
    std::fs::write(output("47-percentages-layout.txt"), format!("{pages:#?}")).unwrap();
    std::fs::write(
        output("47-percentages.docx"),
        super::word::document(&pages).unwrap(),
    )
    .unwrap();
    std::fs::write(
        output("47-percentages.odt"),
        super::odt::document(&pages).unwrap(),
    )
    .unwrap();
}
use super::*;
use crate::drawing::{Drawing, Kind, StrokeStyle};
fn output(name: &str) -> std::path::PathBuf {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test-output");
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}
#[test]
fn word_export_builds_flowing_text_real_table_cells_and_positioned_illustrations() {
    let d = pdfcraft_create::from_text("", "Compte rendu de la classe", (595., 842.), 18.).unwrap();
    let mut e =
        Editor::from_bytes(pdfcraft_cos::write_full(&d, &Default::default()).unwrap()).unwrap();
    let mut drawing = Drawing {
        kind: Kind::FilledRectangle,
        a: [60., 640.],
        u: [480., 0.],
        v: [0., 30.],
        width: 0.7,
        color: [0.75, 0.85, 0.6],
        style: StrokeStyle::Solid,
        rows: 1,
        columns: 1,
    };
    e.add_drawing(0, drawing.clone()).unwrap();
    drawing.kind = Kind::Table;
    drawing.a = [60., 550.];
    drawing.v = [0., 120.];
    drawing.rows = 4;
    drawing.columns = 3;
    drawing.color = [0.; 3];
    e.add_drawing(0, drawing).unwrap();
    for (row, cells) in [
        ["Élève", "Résultat", "Appréciation"],
        ["Alice", "18 / 20", "Très bien"],
        ["Benoît", "14 / 20", "Bon travail"],
        ["Chloé", "16 / 20", "Bravo"],
    ]
    .into_iter()
    .enumerate()
    {
        for (col, text) in cells.into_iter().enumerate() {
            e.add_text(
                0,
                pdfcraft_edit::AddedText {
                    rect: [
                        65. + col as f64 * 160.,
                        640. - row as f64 * 30.,
                        210. + col as f64 * 160.,
                        665. - row as f64 * 30.,
                    ],
                    text: text.into(),
                    size: 11.,
                    bold: row == 0,
                    ..Default::default()
                },
            )
            .unwrap();
        }
    }
    e.add_text(
        0,
        pdfcraft_edit::AddedText {
            rect: [60., 450., 520., 500.],
            text: "Les résultats sont encourageants. Chaque élève progresse à son rythme.".into(),
            size: 12.,
            ..Default::default()
        },
    )
    .unwrap();
    let png = crate::ocr::encode_png(&[30, 140, 230, 255].repeat(100 * 50), 100, 50).unwrap();
    e.image_bytes(0, [60., 310., 260., 410.], "illustration.png", &png, None)
        .unwrap();
    let page = layout::page(&e, 0).unwrap();
    let tables: Vec<_> = page
        .items
        .iter()
        .filter_map(|i| {
            if let Item::Table(t) = i {
                Some(t)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0].rows.len(), 4);
    assert_eq!(tables[0].rows[0].len(), 3);
    assert!(tables[0].rows[0].iter().all(|c| c.fill.is_some()));
    assert!(
        page.decorations.iter().any(|p| p.rect[0] <= 60.
            && p.rect[2] >= 260.
            && p.rect[1] <= 432.
            && p.rect[3] >= 532.
            && p.rect[2] - p.rect[0] < 240.),
        "Keep the inserted illustration beside native text"
    );
    let before = e.bytes.clone();
    std::fs::write(
        output("38-structure.docx"),
        convert(e.bytes.as_ref().clone()).unwrap(),
    )
    .unwrap();
    std::fs::write(
        output("38-structure.odt"),
        convert_as(e.bytes.as_ref().clone(), Format::Odt).unwrap(),
    )
    .unwrap();
    assert_eq!(before, e.bytes);
    e.save(&output("38-structure-source.pdf")).unwrap();
}

#[test]
fn prisms_lesson_keeps_figures_on_the_source_page() {
    let Some(path) = std::env::var_os("PDFINATOR_PRISMS_PDF") else {
        return;
    };
    let before = crate::files::read(std::path::Path::new(&path)).unwrap();
    let e = Editor::from_bytes(before.clone()).unwrap();
    let pages: Vec<_> = (0..e.info.pages.len())
        .map(|i| layout::page(&e, i).unwrap())
        .collect();
    assert_eq!(pages.len(), 1);
    let p = &pages[0];
    assert_eq!(
        p.decorations
            .iter()
            .filter(|i| (i.rect[2] - i.rect[0]) * (i.rect[3] - i.rect[1])
                < p.width * p.height * 0.9)
            .count(),
        3
    );
    assert_eq!(
        p.items
            .iter()
            .filter(|i| matches!(i,Item::Paragraph(p) if dotted(p)))
            .count(),
        7
    );
    assert!(!p.decorations.is_empty());
    assert!(p.items.iter().any(|i| matches!(i,Item::Paragraph(p) if p.runs.iter().map(|r|r.text.as_str()).collect::<String>().starts_with("Chapitre 2"))));
    assert!(
        p.decorations
            .iter()
            .filter(|i| (i.rect[2] - i.rect[0]) * (i.rect[3] - i.rect[1])
                < p.width * p.height * 0.9)
            .all(|i| i.rect[3] < p.height - 1.)
    );

    let mut report = String::new();
    for p in &pages {
        report.push_str(&format!("Page {}x{} {:?}\n", p.width, p.height, p.margin));
        for (i, item) in p.items.iter().enumerate() {
            let label = match item {
                Item::Paragraph(p) => p.runs.iter().map(|r| r.text.as_str()).collect(),
                Item::Table(t) => {
                    format!("Table {}x{} {:?}", t.xs.len() - 1, t.ys.len() - 1, t.rows)
                }
                Item::Picture(_) => "Image".into(),
            };
            report.push_str(&format!("{i}: {:?} {label}\n", item.rect()));
        }
        let mut i = 0;
        while i < p.items.len() {
            let (end, cols) = columns(p, i);
            report.push_str(&format!(
                "Band {i}..{end} {:?}\n",
                cols.iter()
                    .map(|c| c.iter().map(|a| a.rect()).collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            ));
            i = end;
        }
    }
    std::fs::write(output("40-prisms-layout.txt"), report).unwrap();
    std::fs::write(output("40-prisms.docx"), word::document(&pages).unwrap()).unwrap();
    std::fs::write(output("40-prisms.odt"), odt::document(&pages).unwrap()).unwrap();
    assert_eq!(
        crate::files::read(std::path::Path::new(&path)).unwrap(),
        before
    );
}

#[test]
fn worksheet_columns_and_fractions() {
    let Some(path) = std::env::var_os("PDFINATOR_LAYOUT_PDF") else {
        return;
    };
    let before = crate::files::read(std::path::Path::new(&path)).unwrap();
    let e = Editor::from_bytes(before.clone()).unwrap();
    let pages: Vec<_> = (0..e.info.pages.len())
        .map(|i| layout::page(&e, i).unwrap())
        .collect();
    assert_eq!(pages.len(), 1);
    let p = &pages[0];
    if let Some(symbols) = p.fonts.iter().find(|f| f.name.starts_with("OpenSymbol")) {
        let symbols = ttf_parser::Face::parse(&symbols.bytes, 0).unwrap();
        assert!(symbols.glyph_index('=').is_some() && symbols.glyph_index('\u{d7}').is_some());
    }
    let table = p
        .items
        .iter()
        .find_map(|i| {
            if let Item::Table(t) = i {
                Some(t)
            } else {
                None
            }
        })
        .expect("Native source table");
    assert_eq!((table.xs.len() - 1, table.ys.len() - 1), (7, 9));
    let cell_text = |c: &Cell| {
        c.paragraphs
            .iter()
            .flat_map(|p| &p.runs)
            .map(|r| r.text.as_str())
            .collect::<String>()
    };
    assert_eq!(cell_text(&table.rows[0][0]).trim(), "Nombre");
    assert_eq!(cell_text(&table.rows[8][0]).trim(), "4 123");
    let fraction_count = p
        .items
        .iter()
        .filter_map(|i| {
            if let Item::Paragraph(p) = i {
                Some(p)
            } else {
                None
            }
        })
        .flat_map(|p| &p.runs)
        .filter(|r| r.fraction.is_some())
        .count();
    assert!(
        fraction_count > 0
            || p.decorations
                .iter()
                .filter(|pic| pic.rect[1] > table.rect[3] && pic.rect[3] < p.height - 1.)
                .count()
                >= 2,
        "Complex formula rows must be preserved from the source when they cannot stay native"
    );
    assert!(
        p.items
            .iter()
            .filter(|i| matches!(i, Item::Paragraph(_)))
            .count()
            >= 8,
        "The body must remain editable prose"
    );
    let mut i = 0;
    let mut table_band = false;
    while i < p.items.len() {
        let (end, cols) = columns(p, i);
        if cols.iter().flatten().any(|i| matches!(i, Item::Table(_))) {
            assert_eq!(cols.len(), 2);
            assert!(cols[1].iter().any(|i|matches!(i,Item::Paragraph(p) if p.runs.iter().map(|r|r.text.as_str()).collect::<String>().contains("est leur nombre"))));
            table_band = true;
        }
        i = end;
    }
    assert!(table_band);
    let mut report = String::new();
    for p in &pages {
        report.push_str(&format!("Page {}x{} {:?}\n", p.width, p.height, p.margin));
        for (i, item) in p.items.iter().enumerate() {
            let label = match item {
                Item::Paragraph(p) => format!(
                    "{} {:?}",
                    p.runs.iter().map(|r| r.text.as_str()).collect::<String>(),
                    p.runs
                        .iter()
                        .map(|r| (&r.font, r.size, r.rect))
                        .collect::<Vec<_>>()
                ),
                Item::Table(t) => format!("Table {}x{}", t.xs.len() - 1, t.ys.len() - 1),
                Item::Picture(_) => "Image".into(),
            };
            report.push_str(&format!("{i}: {:?} {label}\n", item.rect()));
        }
        let mut i = 0;
        while i < p.items.len() {
            let (end, cols) = columns(p, i);
            report.push_str(&format!(
                "Band {i}..{end} {:?}\n",
                cols.iter()
                    .map(|c| c.iter().map(|a| a.rect()).collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            ));
            i = end;
        }
    }
    std::fs::write(output("39-layout.txt"), report).unwrap();
    std::fs::write(output("39-multiples.docx"), word::document(&pages).unwrap()).unwrap();
    std::fs::write(output("39-multiples.odt"), odt::document(&pages).unwrap()).unwrap();
    assert_eq!(
        crate::files::read(std::path::Path::new(&path)).unwrap(),
        before
    );
}
#[test]
fn user_pdf_exports_all_pages_without_mutating_the_original() {
    let Some(path) = std::env::var_os("PDFINATOR_REGRESSION_PDF") else {
        return;
    };
    let path = std::path::Path::new(&path);
    let before = crate::files::read(path).unwrap();
    let e = Editor::from_bytes(before.clone()).unwrap();
    let pages: Vec<_> = (0..e.info.pages.len())
        .map(|i| layout::page(&e, i).unwrap())
        .collect();
    std::fs::write(output("38-disposition.txt"), format!("{pages:#?}")).unwrap();
    assert_eq!(pages.len(), 2);
    assert!(
        !pages[0].decorations.is_empty(),
        "Keep the ornamental heading frame without rasterizing its words"
    );
    assert!(pages[0].items.iter().any(|i| matches!(i, Item::Paragraph(p) if (0..p.runs.len()).any(|n| angle(&p.runs, n).is_some()))), "Recognize the angle accents as editable equations");
    assert!(
        pages[0]
            .items
            .iter()
            .any(|i| matches!(i, Item::Paragraph(p) if dotted(p))),
        "PDF ellipses must become native dot leaders"
    );
    assert!(
        pages[0]
            .items
            .iter()
            .any(|i| matches!(i, Item::Paragraph(p) if p.runs.iter().any(|r| r.underline))),
        "Source vector underlines remain text formatting"
    );
    for region in [[80., 280., 200., 340.], [293.5, 14.8, 581., 164.2]] {
        assert!(
            pages[1]
                .decorations
                .iter()
                .any(|p| (p.rect[2] - p.rect[0]) * (p.rect[3] - p.rect[1])
                    < pages[1].width * pages[1].height * 0.9
                    && p.rect[0] <= region[0]
                    && p.rect[1] <= region[1]
                    && p.rect[2] >= region[2]
                    && p.rect[3] >= region[3]),
            "A complete curved figure and its labels must travel together: {region:?}"
        );
    }
    assert!(
        pages
            .iter()
            .any(|p| p.items.iter().any(|i| matches!(i, Item::Table(_)))),
        "The grading table should remain a real Word table"
    );
    std::fs::write(
        output("38-document-reel.docx"),
        word::document(&pages).unwrap(),
    )
    .unwrap();
    std::fs::write(
        output("38-document-reel.odt"),
        odt::document(&pages).unwrap(),
    )
    .unwrap();
    assert_eq!(crate::files::read(path).unwrap(), before);
}
