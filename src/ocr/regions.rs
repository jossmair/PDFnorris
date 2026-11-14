use super::{Line, Word};
use crate::document::Result;

#[derive(Clone, Debug)]
pub struct Block {
    pub text: String,
    /// Initially pixels; the desktop converts this to display points once.
    pub rect: [f32; 4],
    pub size: f64,
    pub bold: bool,
    pub color: [f64; 3],
    pub words: Vec<[f32; 4]>,
}
pub(super) fn blocks(lines: &[Line]) -> Vec<Block> {
    let mut result: Vec<Block> = Vec::new();
    for line in lines {
        let words = if line.words.is_empty() {
            vec![Word {
                text: line.text.clone(),
                rect: line.rect,
            }]
        } else {
            line.words.clone()
        };
        let mut current: Option<Block> = None;
        for word in words {
            let height = (word.rect[3] - word.rect[1]).max(1.);
            let join = current.as_ref().is_some_and(|b| {
                word.rect[0] - b.rect[2] < height * 1.7
                    && ((word.rect[1] + word.rect[3] - b.rect[1] - b.rect[3]) / 2.).abs()
                        < height * 0.6
            });
            if !join && let Some(b) = current.take() {
                result.push(b);
            }
            if let Some(b) = &mut current {
                b.text.push(' ');
                b.text.push_str(&word.text);
                b.rect = [
                    b.rect[0].min(word.rect[0]),
                    b.rect[1].min(word.rect[1]),
                    b.rect[2].max(word.rect[2]),
                    b.rect[3].max(word.rect[3]),
                ];
                b.words.push(word.rect);
            } else {
                current = Some(Block {
                    text: word.text,
                    rect: word.rect,
                    size: (height * 1.15) as f64,
                    bold: false,
                    color: [0.; 3],
                    words: vec![word.rect],
                });
            }
        }
        if let Some(b) = current {
            result.push(b);
        }
    }
    result
}
pub fn encode_png(rgba: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder
            .write_header()
            .map_err(|e| e.to_string())?
            .write_image_data(rgba)
            .map_err(|e| e.to_string())?;
    }
    Ok(bytes)
}
/// Change the raster itself, not a white PDF overlay. Only recognised word boxes
/// are reconstructed; all pixels elsewhere stay byte-for-byte unchanged.
pub fn erase_letters(
    rgba: &[u8],
    width: u32,
    height: u32,
    blocks: &mut [Block],
) -> Result<Vec<u8>> {
    let expected = (width as usize)
        .checked_mul(height as usize)
        .and_then(|n| n.checked_mul(4));
    if width == 0 || height == 0 || width > 4096 || height > 4096 || expected != Some(rgba.len()) {
        return Err("Image de reconnaissance invalide.".into());
    }
    let mut result = rgba.to_vec();
    let offset = |x: u32, y: u32| ((y * width + x) * 4) as usize;
    let luma = |p: &[u8]| 0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64;
    for block in blocks {
        let mut ink = [Vec::new(), Vec::new(), Vec::new()];
        for r in &block.words {
            if r.iter().any(|v| !v.is_finite()) {
                continue;
            }
            let x0 = (r[0].floor() - 2.).max(0.).min(width as f32) as u32;
            let y0 = (r[1].floor() - 2.).max(0.).min(height as f32) as u32;
            let x1 = (r[2].ceil() + 2.).max(0.).min(width as f32) as u32;
            let y1 = (r[3].ceil() + 2.).max(0.).min(height as f32) as u32;
            if x1 <= x0 || y1 <= y0 {
                continue;
            }
            let mut border = Vec::new();
            for y in y0.saturating_sub(3)..(y1 + 3).min(height) {
                for x in x0.saturating_sub(3)..(x1 + 3).min(width) {
                    if x < x0 || x >= x1 || y < y0 || y >= y1 {
                        border.push(luma(&rgba[offset(x, y)..]));
                    }
                }
            }
            if border.is_empty() {
                continue;
            }
            border.sort_by(f64::total_cmp);
            let background = border[border.len() / 2];
            let flat = border[border.len() * 9 / 10] - border[border.len() / 10] < 35.;
            let mut low = background;
            let mut high = background;
            for y in y0..y1 {
                for x in x0..x1 {
                    let value = luma(&rgba[offset(x, y)..]);
                    low = low.min(value);
                    high = high.max(value);
                }
            }
            let light = high - background > background - low;
            let contrast = if light {
                high - background
            } else {
                background - low
            };
            if contrast < 25. {
                continue;
            }
            let strength = |x, y| {
                let value = luma(&rgba[offset(x, y)..]);
                if light {
                    value - background
                } else {
                    background - value
                }
            };
            let w = (x1 - x0) as usize;
            let h = (y1 - y0) as usize;
            let mut mask = vec![false; w * h];
            for y in y0..y1 {
                for x in x0..x1 {
                    let value = strength(x, y);
                    if value > (contrast * 0.2).max(18.) {
                        mask[(y - y0) as usize * w + (x - x0) as usize] = true;
                    }
                    if value > contrast * 0.7 {
                        let i = offset(x, y);
                        for c in 0..3 {
                            ink[c].push(rgba[i + c]);
                        }
                    }
                }
            }
            // Cover anti-aliasing and the small shadow immediately around the ink.
            let original = mask.clone();
            for y in 0..h {
                for x in 0..w {
                    if original[y * w + x] {
                        for yy in y.saturating_sub(1)..=(y + 1).min(h - 1) {
                            for xx in x.saturating_sub(1)..=(x + 1).min(w - 1) {
                                mask[yy * w + xx] = true;
                            }
                        }
                    }
                }
            }
            // Only a quiet background can justify restoring a ruled line. Photo
            // highlights at both sides of a word are not table borders.
            if flat {
                let is_ink = |x, y| strength(x, y) > (contrast * 0.35).max(30.);
                for y in y0..y1 {
                    if x0 >= 4
                        && x1 + 4 < width
                        && (1..=4).all(|d| is_ink(x0 - d, y) && is_ink(x1 + d, y))
                    {
                        for x in x0..x1 {
                            mask[(y - y0) as usize * w + (x - x0) as usize] = false;
                        }
                    }
                }
                for x in x0..x1 {
                    if y0 >= 4
                        && y1 + 4 < height
                        && (1..=4).all(|d| is_ink(x, y0 - d) && is_ink(x, y1 + d))
                    {
                        for y in y0..y1 {
                            mask[(y - y0) as usize * w + (x - x0) as usize] = false;
                        }
                    }
                }
            }
            let mut filled: Vec<_> = mask.iter().map(|v| !v).collect();
            let mut queue = std::collections::VecDeque::new();
            for y in 0..h {
                for x in 0..w {
                    if mask[y * w + x]
                        && ((x > 0 && filled[y * w + x - 1])
                            || (x + 1 < w && filled[y * w + x + 1])
                            || (y > 0 && filled[(y - 1) * w + x])
                            || (y + 1 < h && filled[(y + 1) * w + x]))
                    {
                        queue.push_back((x, y));
                    }
                }
            }
            while let Some((x, y)) = queue.pop_front() {
                if filled[y * w + x] {
                    continue;
                }
                let mut sum = [0_u32; 4];
                let mut count = 0;
                for yy in y.saturating_sub(1)..=(y + 1).min(h - 1) {
                    for xx in x.saturating_sub(1)..=(x + 1).min(w - 1) {
                        if filled[yy * w + xx] {
                            let i = offset(x0 + xx as u32, y0 + yy as u32);
                            for c in 0..4 {
                                sum[c] += result[i + c] as u32;
                            }
                            count += 1;
                        }
                    }
                }
                if count == 0 {
                    continue;
                }
                let i = offset(x0 + x as u32, y0 + y as u32);
                for c in 0..4 {
                    result[i + c] = (sum[c] / count) as u8;
                }
                filled[y * w + x] = true;
                for (xx, yy) in [
                    (x.saturating_sub(1), y),
                    ((x + 1).min(w - 1), y),
                    (x, y.saturating_sub(1)),
                    (x, (y + 1).min(h - 1)),
                ] {
                    if !filled[yy * w + xx] {
                        queue.push_back((xx, yy));
                    }
                }
            }
        }
        if !ink[0].is_empty() {
            block.color = std::array::from_fn(|c| {
                ink[c].sort_unstable();
                ink[c][ink[c].len() / 2] as f64 / 255.
            });
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn separate_labels_keep_their_positions_and_the_diagram_remains_in_the_background() {
        let line = Line {
            text: "A 5 cm B".into(),
            rect: [10., 10., 190., 30.],
            words: vec![
                Word {
                    text: "A".into(),
                    rect: [10., 10., 20., 30.],
                },
                Word {
                    text: "5".into(),
                    rect: [60., 10., 70., 30.],
                },
                Word {
                    text: "cm".into(),
                    rect: [75., 10., 95., 30.],
                },
                Word {
                    text: "B".into(),
                    rect: [180., 10., 190., 30.],
                },
            ],
        };
        let mut blocks = blocks(&[line]);
        assert_eq!(
            blocks.iter().map(|b| b.text.as_str()).collect::<Vec<_>>(),
            vec!["A", "5 cm", "B"]
        );
        let mut pixels = vec![255_u8; 200 * 100 * 4];
        for x in 20..180 {
            let i = (60 * 200 + x) * 4;
            pixels[i..i + 3].fill(0);
        }
        for y in 12..28 {
            let i = (y * 200 + 15) * 4;
            pixels[i..i + 3].fill(0);
        }
        let clean = erase_letters(&pixels, 200, 100, &mut blocks).unwrap();
        assert_eq!(
            &clean[(60 * 200 + 80) * 4..(60 * 200 + 80) * 4 + 3],
            &[0, 0, 0]
        );
        assert_eq!(
            &clean[(20 * 200 + 15) * 4..(20 * 200 + 15) * 4 + 3],
            &[255, 255, 255]
        );
        for y in 31..100 {
            assert_eq!(
                &clean[y * 200 * 4..(y + 1) * 200 * 4],
                &pixels[y * 200 * 4..(y + 1) * 200 * 4]
            );
        }
    }
}

#[cfg(test)]
mod photo_regressions {
    use super::*;
    fn sample() -> (Vec<u8>, Block) {
        let mut pixels = vec![255_u8; 80 * 48 * 4];
        for y in 0..48 {
            for x in 0..80 {
                let i = (y * 80 + x) * 4;
                let shade = (24 + x / 4 + y / 6) as u8;
                pixels[i..i + 3].fill(shade);
            }
        }
        for y in 12..28 {
            for x in 15..20 {
                let i = (y * 80 + x) * 4;
                pixels[i..i + 3].fill(245);
            }
        }
        let b = Block {
            text: "Ici".into(),
            rect: [10., 10., 42., 30.],
            size: 20.,
            bold: false,
            color: [0.; 3],
            words: vec![[10., 10., 42., 30.]],
        };
        (pixels, b)
    }
    #[test]
    fn white_text_keeps_a_light_editable_colour_on_a_dark_photo() {
        let (pixels, b) = sample();
        let mut blocks = vec![b];
        let clean = erase_letters(&pixels, 80, 48, &mut blocks).unwrap();
        assert!(
            blocks[0].color.iter().all(|v| *v > 0.9),
            "{:?}",
            blocks[0].color
        );
        assert!(
            clean[(20 * 80 + 17) * 4] < 100,
            "Old white letters must not remain behind new text"
        );
    }
    #[test]
    fn removing_letters_does_not_flatten_the_rest_of_the_photo_word_box() {
        let (pixels, b) = sample();
        let mut blocks = vec![b];
        let clean = erase_letters(&pixels, 80, 48, &mut blocks).unwrap();
        let untouched = (20 * 80 + 34) * 4;
        assert_eq!(
            &clean[untouched..untouched + 4],
            &pixels[untouched..untouched + 4]
        );
    }
}
