use crate::index_roots;
use crate::search::{DocumentIndexer, SearchEngine, SearchResult};
use crate::ui::search_results_page::{self, SerpAction};
use egui::{
    Align, CentralPanel, Color32, Context, FontId, Frame, Layout, Modifiers, RichText, Rounding,
    ScrollArea, Stroke, TextEdit, TextStyle, Vec2,
};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;

fn search_field_id() -> egui::Id {
    egui::Id::new("everywhere_search_field")
}

fn apply_theme_once(ctx: &Context) {
    static ONCE: OnceLock<()> = OnceLock::new();
    if ONCE.set(()).is_err() {
        return;
    }
    let mut style = (*ctx.style()).clone();
    let soft = Rounding::same(8.0);
    style.visuals.widgets.noninteractive.rounding = soft;
    for w in [
        &mut style.visuals.widgets.inactive,
        &mut style.visuals.widgets.hovered,
        &mut style.visuals.widgets.active,
    ] {
        w.rounding = soft;
        w.bg_stroke = Stroke::new(1.0, Color32::from_rgb(230, 232, 236));
    }
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(255, 255, 255);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(250, 250, 252);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(246, 247, 249);
    style.spacing.button_padding = Vec2::new(20.0, 9.0);
    style.spacing.item_spacing = Vec2::new(8.0, 10.0);
    let prop = egui::FontFamily::Proportional;
    let mono = egui::FontFamily::Monospace;
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(16.0, prop.clone()));
    style
        .text_styles
        .insert(TextStyle::Button, FontId::new(14.0, prop.clone()));
    style
        .text_styles
        .insert(TextStyle::Small, FontId::new(13.0, prop.clone()));
    style
        .text_styles
        .insert(TextStyle::Monospace, FontId::new(13.0, mono));
    ctx.set_style(style);
}

enum ActiveView {
    Home,
    Serp,
}

pub struct MainWindow {
    search_engine: Arc<parking_lot::Mutex<SearchEngine>>,
    indexer: Arc<DocumentIndexer>,
    index_dir: PathBuf,
    indexed_roots: Vec<PathBuf>,
    search_query: String,
    search_results: Vec<SearchResult>,
    is_searching: bool,
    index_done_rx: Option<Receiver<Result<(), String>>>,
    index_status: String,
    indexing: bool,
    active_view: ActiveView,
}

impl MainWindow {
    pub fn new(
        search_engine: Arc<parking_lot::Mutex<SearchEngine>>,
        indexer: Arc<DocumentIndexer>,
        index_dir: PathBuf,
    ) -> Self {
        let indexed_roots = index_roots::load_roots(&index_dir);
        Self {
            search_engine,
            indexer,
            index_dir,
            indexed_roots,
            search_query: String::new(),
            search_results: Vec::new(),
            is_searching: false,
            index_done_rx: None,
            index_status: "Choose a folder to build the search index.".to_string(),
            indexing: false,
            active_view: ActiveView::Home,
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        apply_theme_once(ctx);

        let sid = search_field_id();
        if ctx.memory(|m| m.focus()) == Some(sid)
            && ctx.input_mut(|i| i.consume_key(Modifiers::NONE, egui::Key::Enter))
            && !self.search_query.trim().is_empty()
        {
            self.perform_search();
            self.active_view = ActiveView::Serp;
        }

        if self.indexing {
            ctx.request_repaint_after(Duration::from_millis(200));
        }

        if let Some(rx) = &self.index_done_rx {
            match rx.try_recv() {
                Ok(Ok(())) => {
                    self.index_done_rx = None;
                    self.indexing = false;
                    self.index_status = "Index updated. You can search now.".to_string();
                    self.indexed_roots = index_roots::load_roots(&self.index_dir);
                }
                Ok(Err(e)) => {
                    self.index_done_rx = None;
                    self.indexing = false;
                    self.index_status = format!("Indexing failed: {}", e);
                    tracing::error!("{}", e);
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.index_done_rx = None;
                    self.indexing = false;
                    self.index_status = "Indexing task ended unexpectedly.".to_string();
                }
            }
        }

        let page = Color32::from_rgb(252, 252, 250);
        let serp_bg = Color32::WHITE;
        CentralPanel::default()
            .frame(
                Frame::central_panel(ctx.style().as_ref()).fill(match self.active_view {
                    ActiveView::Home => page,
                    ActiveView::Serp => serp_bg,
                }),
            )
            .show(ctx, |ui| {
                match self.active_view {
                    ActiveView::Home => {
                        ui.vertical_centered(|ui| {
                            ui.set_max_width(720.0);

                            ui.add_space(48.0);

                            ui.horizontal(|ui| {
                                let word_w = 9.0 * 22.0;
                                ui.add_space((ui.available_width() - word_w).max(0.0) * 0.5);
                                ui.horizontal(|ui| {
                                    let palette: &[(u8, u8, u8)] = &[
                                        (66, 133, 244),
                                        (234, 67, 53),
                                        (251, 188, 5),
                                        (52, 168, 83),
                                    ];
                                    for (i, ch) in "Everywhere".chars().enumerate() {
                                        let (r, g, b) = palette[i % palette.len()];
                                        ui.push_id(i, |ui| {
                                            ui.label(
                                                RichText::new(ch.to_string())
                                                    .size(34.0)
                                                    .strong()
                                                    .color(Color32::from_rgb(r, g, b)),
                                            );
                                        });
                                    }
                                });
                            });

                            ui.add_space(26.0);

                            let search_w = 560.0_f32;
                            ui.horizontal(|ui| {
                                ui.add_space((ui.available_width() - search_w).max(0.0) * 0.5);
                                ui.add(
                                    TextEdit::singleline(&mut self.search_query)
                                        .id(search_field_id())
                                        .hint_text("Search indexed files")
                                        .desired_width(search_w)
                                        .frame(true)
                                        .margin(Vec2::new(16.0, 14.0)),
                                );
                            });

                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("Press Enter to open results")
                                    .small()
                                    .color(Color32::from_gray(130)),
                            );

                            ui.add_space(18.0);

                            ui.horizontal(|ui| {
                                let row_w = 132.0_f32;
                                ui.add_space((ui.available_width() - row_w).max(0.0) * 0.5);
                                if ui
                                    .add_sized(Vec2::new(132.0, 36.0), egui::Button::new("Index folder"))
                                    .clicked()
                                {
                                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                        self.start_index(path);
                                    }
                                }
                            });

                            ui.add_space(10.0);
                            ui.label(
                                RichText::new(&self.index_status)
                                    .small()
                                    .color(Color32::from_gray(100)),
                            );

                            if !self.indexed_roots.is_empty() {
                                ui.add_space(20.0);
                                ui.label(
                                    RichText::new("Indexed folders (top-level paths)")
                                        .small()
                                        .strong()
                                        .color(Color32::from_gray(55)),
                                );
                                ui.add_space(6.0);
                                ScrollArea::vertical()
                                    .id_source("everywhere_indexed_roots")
                                    .max_height(100.0)
                                    .show(ui, |ui| {
                                        ui.set_max_width(640.0);
                                        for p in &self.indexed_roots {
                                            ui.label(
                                                RichText::new(p.display().to_string())
                                                    .small()
                                                    .monospace()
                                                    .color(Color32::from_gray(75)),
                                            );
                                        }
                                    });
                            }

                            ui.add_space(28.0);
                            ui.label(
                                RichText::new(
                                    "Index a folder, then search across PDF, Office, Markdown, and text files.",
                                )
                                .small()
                                .color(Color32::from_gray(130)),
                            );
                        });
                    }
                    ActiveView::Serp => {
                        ui.with_layout(Layout::top_down(Align::Min), |ui| {
                            ui.add_space(20.0);
                            ui.horizontal(|ui| {
                                ui.add_space(24.0);
                                ui.vertical(|ui| {
                                    let action = search_results_page::show(
                                        ui,
                                        search_field_id(),
                                        &mut self.search_query,
                                        &self.search_results,
                                        self.is_searching,
                                    );
                                    match action {
                                        SerpAction::None => {}
                                        SerpAction::SubmitSearch => {
                                            if !self.search_query.trim().is_empty() {
                                                self.perform_search();
                                            }
                                        }
                                        SerpAction::OpenFile(i) => {
                                            if let Some(r) = self.search_results.get(i) {
                                                self.open_file(&r.path);
                                            }
                                        }
                                        SerpAction::GoHome => {
                                            self.active_view = ActiveView::Home;
                                        }
                                    }
                                });
                            });
                        });
                    }
                }
            });
    }

    fn perform_search(&mut self) {
        if self.search_query.trim().is_empty() {
            return;
        }
        self.is_searching = true;
        match self.search_engine.lock().search(&self.search_query, 20) {
            Ok(results) => {
                self.search_results = results;
            }
            Err(e) => {
                tracing::error!("search failed: {}", e);
                self.search_results.clear();
            }
        }
        self.is_searching = false;
    }

    fn start_index(&mut self, path: std::path::PathBuf) {
        if self.indexing {
            self.index_status = "Indexing is already in progress.".to_string();
            return;
        }
        self.indexing = true;
        self.index_status = format!("Indexing: {}", path.display());
        let (tx, rx) = std::sync::mpsc::channel();
        let indexer = Arc::clone(&self.indexer);
        std::thread::spawn(move || {
            let rt = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(e) => {
                    let _ = tx.send(Err(e.to_string()));
                    return;
                }
            };
            let res = rt.block_on(indexer.index_directory(&path));
            let _ = tx.send(res.map_err(|e| e.to_string()));
        });
        self.index_done_rx = Some(rx);
    }

    fn open_file(&self, path: &std::path::Path) {
        if let Err(e) = opener::open(path) {
            tracing::error!("open file failed: {}", e);
        }
    }
}
