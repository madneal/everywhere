use crate::search::SearchResult;
use egui::{
    Align, Color32, CursorIcon, FontId, Frame, Layout, Margin, RichText, Rounding, ScrollArea,
    Sense, Stroke, TextEdit, Ui, Vec2,
};
use egui::text::{LayoutJob, TextFormat};
use std::path::Path;

const TITLE_BLUE: Color32 = Color32::from_rgb(26, 13, 171);
const URL_GREEN: Color32 = Color32::from_rgb(0, 104, 55);
const SNIPPET_GRAY: Color32 = Color32::from_rgb(77, 81, 86);
const META_GRAY: Color32 = Color32::from_rgb(112, 117, 122);
const SOURCE_TITLE: Color32 = Color32::from_rgb(32, 33, 36);

pub enum SerpAction {
    None,
    SubmitSearch,
    OpenFile(usize),
    GoHome,
}

fn query_terms(query: &str) -> Vec<String> {
    let q = query.trim();
    if q.is_empty() {
        return Vec::new();
    }
    if q.chars().any(|c| c.is_ascii_whitespace()) {
        q.split_whitespace().map(|s| s.to_string()).filter(|s| !s.is_empty()).collect()
    } else {
        vec![q.to_string()]
    }
}

fn find_match(haystack: &str, term: &str) -> Option<usize> {
    if term.is_empty() {
        return None;
    }
    if term.is_ascii() {
        for (idx, _) in haystack.char_indices() {
            if let Some(tail) = haystack.get(idx..) {
                if tail.len() >= term.len() {
                    let slice = &tail[..term.len()];
                    if slice.eq_ignore_ascii_case(term) {
                        return Some(idx);
                    }
                }
            }
        }
        None
    } else {
        haystack.find(term)
    }
}

fn merge_byte_spans(mut spans: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    spans.sort_by_key(|s| s.0);
    let mut out: Vec<(usize, usize)> = Vec::new();
    for (s, e) in spans {
        if s >= e {
            continue;
        }
        if let Some(last) = out.last_mut() {
            if s <= last.1 {
                last.1 = last.1.max(e);
            } else {
                out.push((s, e));
            }
        } else {
            out.push((s, e));
        }
    }
    out
}

fn sanitize_spans(text_len: usize, spans: &[(usize, usize)]) -> Vec<(usize, usize)> {
    spans
        .iter()
        .filter_map(|&(s, e)| {
            if s < e && s < text_len {
                Some((s, e.min(text_len)))
            } else {
                None
            }
        })
        .collect()
}

fn snippet_layout_job_ranges(text: &str, ranges: &[(usize, usize)], font: FontId) -> LayoutJob {
    let normal = TextFormat {
        font_id: font.clone(),
        color: SNIPPET_GRAY,
        ..Default::default()
    };
    let hi = TextFormat {
        font_id: FontId::new(14.0, font.family.clone()),
        color: Color32::from_rgb(32, 33, 36),
        ..Default::default()
    };
    let merged = merge_byte_spans(ranges.to_vec());
    let mut job = LayoutJob::default();
    let mut cursor = 0usize;
    for (s, e) in merged {
        if cursor < s && s <= text.len() {
            job.append(&text[cursor..s], 0.0, normal.clone());
        }
        if s < e && e <= text.len() {
            job.append(&text[s..e], 0.0, hi.clone());
        }
        cursor = cursor.max(e);
    }
    if cursor < text.len() {
        job.append(&text[cursor..], 0.0, normal);
    }
    job
}

fn snippet_layout_for_excerpt(
    text: &str,
    query: &str,
    engine_spans: &[(usize, usize)],
    font: FontId,
) -> LayoutJob {
    let cleaned = sanitize_spans(text.len(), engine_spans);
    let merged = merge_byte_spans(cleaned);
    if !merged.is_empty() {
        snippet_layout_job_ranges(text, &merged, font)
    } else {
        snippet_layout_job(text, query, font)
    }
}

fn merged_highlight_ranges(text: &str, query: &str) -> Vec<(usize, usize)> {
    let terms = query_terms(query);
    let mut spans: Vec<(usize, usize)> = Vec::new();
    for term in terms {
        let mut from = 0;
        while let Some(rel) = find_match(&text[from..], &term) {
            let start = from + rel;
            let end = start + term.len();
            spans.push((start, end));
            from = start + term.chars().next().map(|c| c.len_utf8()).unwrap_or(1);
        }
    }
    spans.sort_by_key(|&(s, _)| s);
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (s, e) in spans {
        if let Some(last) = merged.last_mut() {
            if s <= last.1 {
                last.1 = last.1.max(e);
            } else {
                merged.push((s, e));
            }
        } else {
            merged.push((s, e));
        }
    }
    merged
}

fn snippet_layout_job(text: &str, query: &str, font: FontId) -> LayoutJob {
    let normal = TextFormat {
        font_id: font.clone(),
        color: SNIPPET_GRAY,
        ..Default::default()
    };
    let hi = TextFormat {
        font_id: FontId::new(14.0, font.family.clone()),
        color: Color32::from_rgb(32, 33, 36),
        ..Default::default()
    };
    let ranges = merged_highlight_ranges(text, query);
    let mut job = LayoutJob::default();
    let mut cursor = 0usize;
    for (s, e) in ranges {
        if cursor < s && s <= text.len() {
            job.append(&text[cursor..s], 0.0, normal.clone());
        }
        if s < e && e <= text.len() {
            job.append(&text[s..e], 0.0, hi.clone());
        }
        cursor = cursor.max(e);
    }
    if cursor < text.len() {
        job.append(&text[cursor..], 0.0, normal);
    }
    job
}

fn source_label(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| path.display().to_string())
}

fn favicon_color(path: &Path) -> Color32 {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let h = name.bytes().fold(0u32, |a, b| a.wrapping_add(b as u32));
    let r = 120 + (h % 100) as u8;
    let g = 120 + ((h / 7) % 100) as u8;
    let b = 120 + ((h / 13) % 100) as u8;
    Color32::from_rgb(r, g, b)
}

fn favicon_letter(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .and_then(|s| s.chars().next())
        .map(|c| c.to_uppercase().collect::<String>())
        .unwrap_or_else(|| "?".to_string())
}

pub fn show(
    ui: &mut Ui,
    _search_field_id: egui::Id,
    query: &mut String,
    results: &[SearchResult],
    is_searching: bool,
) -> SerpAction {
    let mut action = SerpAction::None;
    let full = ui.available_width();
    ui.set_max_width((full - 48.0).min(720.0));

    ui.horizontal(|ui| {
        ui.add_space(4.0);
        if ui
            .link(RichText::new("Everywhere").size(18.0).strong().color(TITLE_BLUE))
            .clicked()
        {
            action = SerpAction::GoHome;
        }
    });

    ui.add_space(12.0);

    let bar_h = 46.0;
    Frame::none()
        .fill(Color32::WHITE)
        .stroke(Stroke::new(1.0, Color32::from_gray(210)))
        .rounding(Rounding::same(24.0))
        .inner_margin(Margin::symmetric(18.0, 0.0))
        .show(ui, |ui| {
            ui.set_min_height(bar_h);
            ui.horizontal_centered(|ui| {
                ui.add(
                    TextEdit::singleline(query)
                        .id(_search_field_id)
                        .frame(false)
                        .hint_text("Search")
                        .desired_width(ui.available_width() - 56.0),
                );
                if ui
                    .add_sized(Vec2::new(40.0, bar_h - 8.0), egui::Button::new("🔍").small())
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    action = SerpAction::SubmitSearch;
                }
            });
        });

    ui.add_space(20.0);

    if is_searching {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label(RichText::new("Searching…").color(META_GRAY));
        });
        return action;
    }

    if results.is_empty() {
        ui.label(
            RichText::new("No results found.")
                .size(15.0)
                .color(META_GRAY),
        );
        return action;
    }

    let body_font = FontId::proportional(14.0);
    let avail_h = ui.available_height().max(160.0);
    ScrollArea::vertical()
        .id_source("everywhere_serp_results")
        .max_height(avail_h)
        .show(ui, |ui| {
            ui.set_max_width(680.0);
            for (i, result) in results.iter().enumerate() {
                ui.add_space(18.0);
                ui.horizontal_top(|ui| {
                    let letter = favicon_letter(&result.path);
                    let fav = favicon_color(&result.path);
                    Frame::none()
                        .fill(fav.linear_multiply(0.35))
                        .stroke(Stroke::new(1.0, Color32::from_gray(220)))
                        .rounding(Rounding::same(10.0))
                        .inner_margin(Margin::same(6.0))
                        .show(ui, |ui| {
                            ui.set_min_size(Vec2::new(28.0, 28.0));
                            ui.centered_and_justified(|ui| {
                                ui.label(
                                    RichText::new(letter)
                                        .size(13.0)
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                        });
                    ui.add_space(12.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(source_label(&result.path))
                                    .size(14.0)
                                    .color(SOURCE_TITLE),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(RichText::new("⋮").color(META_GRAY));
                            });
                        });
                        ui.add_space(2.0);
                        let path_str = result.path.display().to_string();
                        if ui
                            .link(
                                RichText::new(&path_str)
                                    .size(12.0)
                                    .monospace()
                                    .color(URL_GREEN),
                            )
                            .on_hover_text("Open file")
                            .clicked()
                        {
                            action = SerpAction::OpenFile(i);
                        }
                        ui.add_space(4.0);
                        let title = RichText::new(&result.title)
                            .size(20.0)
                            .strong()
                            .color(TITLE_BLUE);
                        if ui.link(title).clicked() {
                            action = SerpAction::OpenFile(i);
                        }
                        ui.add_space(4.0);
                        let job = snippet_layout_for_excerpt(
                            &result.content,
                            query,
                            &result.snippet_highlights,
                            body_font.clone(),
                        );
                        let sense = Sense::click();
                        let r = ui.add(egui::Label::new(job).sense(sense));
                        if r.clicked() {
                            action = SerpAction::OpenFile(i);
                        }
                        r.on_hover_cursor(CursorIcon::PointingHand);
                    });
                });
            }
        });

    action
}
