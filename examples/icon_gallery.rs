//! Render the SVG icon family at UI size and at 2x for local visual review.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::args()
        .nth(1)
        .ok_or("Usage: icon_gallery OUTPUT.png")?;
    let mut files: Vec<_> = std::fs::read_dir("assets/icons")?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "svg"))
        .collect();
    files.sort();
    let poster = std::env::args().nth(2).as_deref() == Some("poster");
    let (cell_width, cell_height) = if poster { (384, 360) } else { (120, 100) };
    let cols = if poster { 10 } else { 8 };
    let rows = files.len().div_ceil(cols);
    let mut canvas =
        resvg::tiny_skia::Pixmap::new((cols * cell_width) as u32, (rows * cell_height) as u32)
            .unwrap();
    canvas.fill(if std::env::args().nth(2).as_deref() == Some("light") {
        resvg::tiny_skia::Color::from_rgba8(246, 248, 252, 255)
    } else {
        resvg::tiny_skia::Color::from_rgba8(16, 25, 42, 255)
    });
    for (i, path) in files.iter().enumerate() {
        let tree = resvg::usvg::Tree::from_data(&std::fs::read(path)?, &Default::default())?;
        let (x, y) = ((i % cols) * cell_width, (i / cols) * cell_height);
        let scales = if poster {
            vec![(64., 8.)]
        } else {
            vec![(8., 1.), (48., 2.)]
        };
        for (dx, scale) in scales {
            resvg::render(
                &tree,
                resvg::tiny_skia::Transform::from_scale(scale, scale)
                    .post_translate(x as f32 + dx, y as f32 + if poster { 52. } else { 15. }),
                &mut canvas.as_mut(),
            );
        }
        println!("{}: {}", i + 1, path.display());
    }
    canvas.save_png(out)?;
    Ok(())
}
