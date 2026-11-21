//! Shared reading order and paragraph flow for Word and OpenDocument.
use super::*;

pub(super) fn is_footer(page: &Page, item: &Item) -> bool {
    let Item::Paragraph(p) = item else {
        return false;
    };
    p.runs.iter().any(|r| !r.text.trim().is_empty())
        && p.rect[1] > page.height * 0.94
        && p.runs.iter().map(|r| r.size).fold(0., f64::max) <= 11.
        && p.runs.iter().map(|r| r.text.chars().count()).sum::<usize>() <= 120
        && page.items.iter().any(|i| i.rect()[1] < p.rect[1] - 18.)
        && page
            .items
            .iter()
            .filter(|i| i.rect()[1] < p.rect[1] - 1.)
            .all(|i| i.rect()[3] <= p.rect[1] - 18.)
}
pub(super) fn body_without_footers(page: &Page) -> Option<Page> {
    if !page.items.iter().any(|i| is_footer(page, i)) {
        return None;
    }
    Some(Page {
        width: page.width,
        height: page.height,
        margin: page.margin,
        items: page
            .items
            .iter()
            .filter(|i| !is_footer(page, i))
            .cloned()
            .collect(),
        decorations: page.decorations.clone(),
        fonts: vec![],
    })
}
pub(super) fn dotted(p: &Paragraph) -> bool {
    leader_prefix(p).is_some()
}
pub(super) fn leader_prefix(p: &Paragraph) -> Option<Vec<Run>> {
    let text: String = p.runs.iter().map(|r| r.text.as_str()).collect();
    let prefix = text.trim_end_matches(|c: char| matches!(c, '.' | '…' | '·') || c.is_whitespace());
    if text[prefix.len()..]
        .chars()
        .filter(|c| !c.is_whitespace())
        .count()
        <= 8
    {
        return None;
    }
    let mut end = prefix.len();
    Some(
        p.runs
            .iter()
            .filter_map(|r| {
                if end == 0 {
                    return None;
                }
                let mut r = r.clone();
                r.text.truncate(end.min(r.text.len()));
                end -= r.text.len();
                Some(r)
            })
            .collect(),
    )
}
pub(super) fn angle(runs: &[Run], i: usize) -> Option<&Run> {
    let marker = runs.get(i)?;
    let base = runs.get(i + 1)?;
    let value = base.text.trim();
    (marker.text.trim() == "^"
        && (2..=4).contains(&value.len())
        && value.chars().all(|c| c.is_ascii_uppercase())
        && marker
            .rect
            .zip(base.rect)
            .is_some_and(|(a, b)| a[0] < b[2] && b[0] < a[2]))
    .then_some(base)
}

// Track the height actually emitted by the word processor, rather than the
// source font's em box. Mixing these two caused cumulative vertical drift.
pub(super) fn flow_height(item: &Item) -> f64 {
    match item {
        Item::Paragraph(p) => paragraph_height(p),
        Item::Picture(p) => p.rect[3] - p.rect[1] + 0.2,
        Item::Table(t) => t.rect[3] - t.rect[1] + 0.5,
    }
}
pub(super) fn paragraph_height(p: &Paragraph) -> f64 {
    if p.runs.iter().any(|r| r.fraction.is_some()) {
        p.rect[3] - p.rect[1]
    } else {
        p.runs.iter().map(|r| r.size).fold(1., f64::max)
    }
}
pub(super) fn math_parts(p: &Paragraph) -> Vec<Paragraph> {
    let mut parts: Vec<Paragraph> = Vec::new();
    for r in &p.runs {
        let rect = r.rect.unwrap_or(p.rect);
        if r.fraction.is_none()
            && let Some(last) = parts.last_mut()
            && last.runs.last().is_some_and(|r| r.fraction.is_none())
            && rect[0] - last.rect[2] < r.size * 1.5
        {
            last.rect = [
                last.rect[0].min(rect[0]),
                last.rect[1].min(rect[1]),
                last.rect[2].max(rect[2]),
                last.rect[3].max(rect[3]),
            ];
            last.runs.push(r.clone());
        } else {
            parts.push(Paragraph {
                rect,
                runs: vec![r.clone()],
                align: "left",
                spacing: None,
            });
        }
    }
    parts
}
pub(super) fn tab_before(runs: &[Run], i: usize) -> bool {
    i > 0
        && runs[i].rect.zip(runs[i - 1].rect).is_some_and(|(b, a)| {
            let math_token = |r: &Run| {
                !r.font.starts_with("Arial")
                    && r.text
                        .chars()
                        .any(|c| c.is_ascii_digit() || "=×÷+−".contains(c))
                    && r.text.chars().all(|c| {
                        c.is_ascii_digit() || c.is_whitespace() || ".,/=×÷+−-…%".contains(c)
                    })
            };
            if runs.first().is_some_and(math_token)
                // Compact equations already fit their source advances. Tabs
                // between their tightly spaced operators can skip a stop.
                && runs.iter().any(|r| r.font.starts_with("Arial") || r.size >= 14.)
                && math_token(&runs[i])
                && math_token(&runs[i - 1])
                && runs[i].script == 0
                && runs[i - 1].script == 0
                && b[0] >= a[2] - 0.5
            {
                return true;
            }
            let math_to_prose = runs[i].font.starts_with("Arial")
                && !runs[i - 1].font.starts_with("Arial")
                && runs[i].script == 0
                && runs[i - 1].script == 0
                && runs[i]
                    .text
                    .trim_start()
                    .chars()
                    .next()
                    .is_some_and(char::is_alphabetic)
                && runs[i - 1]
                    .text
                    .trim_end()
                    .chars()
                    .last()
                    .is_some_and(|c| c.is_ascii_digit());
            if math_to_prose && b[0] >= a[2] - 0.5 {
                return true;
            }
            b[0] - a[2]
                > if runs[i].fraction.is_some() || runs[i - 1].fraction.is_some() {
                    2.
                } else {
                    runs[i].size * 0.45
                }
        })
}

/// One band of parallel columns, including following paragraphs beside a tall
/// illustration. Both formats share this reading order and column structure.
pub(super) fn columns(page: &Page, start: usize) -> (usize, Vec<Vec<&Item>>) {
    let first = &page.items[start];
    let r = first.rect();
    let mut end = start + 1;
    while end < page.items.len() && end - start < 8 {
        let next = page.items[end].rect();
        if ((next[1] - r[1]).abs() > 14. && !(matches!(first, Item::Picture(_)) && next[1] < r[3]))
            || page.items[start..end].iter().any(|a| {
                let a = a.rect();
                a[0] + 1. < next[2] && next[0] + 1. < a[2]
            })
        {
            break;
        }
        end += 1;
    }
    let mut cols: Vec<Vec<&Item>> = page.items[start..end].iter().map(|i| vec![i]).collect();
    cols.sort_by(|a, b| a[0].rect()[0].total_cmp(&b[0].rect()[0]));
    // A tall figure can face a text column that itself contains a small
    // illustration. Those short side-by-side items form one nested band.
    if cols.len() >= 3 {
        let tall = cols
            .iter()
            .enumerate()
            .filter(|(_, c)| matches!(c[0], Item::Picture(_)))
            .max_by(|(_, a), (_, b)| {
                (a[0].rect()[3] - a[0].rect()[1]).total_cmp(&(b[0].rect()[3] - b[0].rect()[1]))
            })
            .map(|(n, c)| (n, c[0].rect()[3] - c[0].rect()[1]));
        if let Some((n, h)) = tall.filter(|(_, h)| *h > 80.) {
            let short = |c: &Vec<&Item>| c[0].rect()[3] - c[0].rect()[1] < h * 0.6;
            if n >= 2 && cols[..n].iter().all(short) {
                let merged = cols.drain(..n).flatten().collect();
                cols.insert(0, merged);
            } else if cols.len() - n > 2 && cols[n + 1..].iter().all(short) {
                let merged = cols.drain(n + 1..).flatten().collect();
                cols.push(merged);
            }
        }
    }
    if cols.len() > 1 {
        let mut bottom = cols.iter().map(|c| c[0].rect()[3]).fold(r[3], f64::max);
        while end < page.items.len() && page.items[end].rect()[1] < bottom + 15. {
            let item = &page.items[end];
            let rect = item.rect();
            let col = cols.iter().enumerate().position(|(n, c)| {
                rect[0]
                    >= if n == 0 {
                        page.margin[0] - 2.
                    } else {
                        (cols[n - 1][0].rect()[2] + c[0].rect()[0]) / 2. - 3.
                    }
                    && rect[2]
                        <= cols
                            .get(n + 1)
                            .map_or(page.width - page.margin[2], |c| c[0].rect()[0])
                            + 2.
            });
            if let Some(col) = col {
                bottom = bottom.max(rect[3]);
                cols[col].push(item);
                end += 1;
            } else {
                break;
            }
        }
    }
    (end, cols)
}
