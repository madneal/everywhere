use crate::search::SearchResult;
use egui::{
    Align, Color32, CursorIcon, FontId, Frame, Layout, Margin, RichText, Rounding, ScrollArea, Sense,
    Stroke, TextEdit, TextFormat, Ui, Vec2,
};
use egui::text::LayoutJob;
use std::path::Path;

const SNIPPET_DISPLAY_MAX_CHARS: usize = 800;

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

fn clip_highlights(hls: &[(usize, usize)], end_byte: usize) -> Vec<(usize, usize)> {
    hls.iter()
        .filter_map(|&(s, e)| {
            if s >= end_byte {
                return None;
            }
            Some((s, e.min(end_byte)))
        })
        .collect()
}

fn snippet_layout_job(content: &str, highlights: &[(usize, usize)]) -> LayoutJob {
    let font = FontId::proportional(14.0);
    let normal = TextFormat {
        font_id: font.clone(),
        color: SNIPPET_GRAY,
        ..Default::default()
    };
    let hi = TextFormat {
        font_id: font,
        color: TITLE_BLUE,
        ..Default::default()
    };
    let mut job = LayoutJob::default();
    let mut hls: Vec<(usize, usize)> = highlights
        .iter()
        .filter(|(s, e)| s < e && *e <= content.len())
        .copied()
        .collect();
    hls.sort_by_key(|x| x.0);
    let mut pos = 0usize;
    for (s, e) in hls {
        let s = s.max(pos);
        if s >= content.len() {
            break;
        }
        let e = e.min(content.len());
        if s > pos {
            job.append(&content[pos..s], 0.0, normal.clone());
        }
        if e > s {
            job.append(&content[s..e], 0.0, hi.clone());
        }
        pos = e;
    }
    if pos < content.len() {
        job.append(&content[pos..], 0.0, normal);
    }
    job
}

fn paint_snippet(ui: &mut Ui, content: &str, highlights: &[(usize, usize)]) -> bool {
    let w = ui.available_width().max(1.0);
    if content.trim().is_empty() {
        ui.label(
            RichText::new("（未读取到正文，请确认文件在本地且路径与索引一致）")
                .size(13.0)
                .color(META_GRAY),
        );
        return false;
    }
    let mut end = content.len();
    if content.chars().count() > SNIPPET_DISPLAY_MAX_CHARS {
        end = content
            .char_indices()
            .nth(SNIPPET_DISPLAY_MAX_CHARS)
            .map(|(b, _)| b)
            .unwrap_or(content.len());
    }
    let slice = content.get(..end).unwrap_or(content);
    let hls = clip_highlights(highlights, end);
    let job = snippet_layout_job(slice, &hls);
    ui.set_width(w);
    ui.add(
        egui::Label::new(job)
            .wrap(true)
            .sense(Sense::click()),
    )
    .on_hover_cursor(CursorIcon::PointingHand)
    .clicked()
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

    let avail_h = ui.available_height().max(160.0);
    ScrollArea::vertical()
        .id_source("everywhere_serp_results")
        .max_height(avail_h)
        .show(ui, |ui| {
            ui.set_max_width(680.0);
            for (i, result) in results.iter().enumerate() {
                ui.add_space(18.0);
                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(40.0);
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
                    });
                    ui.add_space(12.0);
                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width());
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
                        if paint_snippet(ui, &result.content, &result.snippet_highlights) {
                            action = SerpAction::OpenFile(i);
                        }
                    });
                });
            }
        });

    action
}
