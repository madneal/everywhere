use crate::search::{SearchEngine, SearchResult};
use std::sync::Arc;
use egui::{Context, TopBottomPanel, CentralPanel, ScrollArea};

pub struct MainWindow {
    search_engine: Arc<SearchEngine>,
    search_query: String,
    search_results: Vec<SearchResult>,
    is_searching: bool,
}

impl MainWindow {
    pub fn new(search_engine: Arc<SearchEngine>) -> Self {
        Self {
            search_engine,
            search_query: String::new(),
            search_results: Vec::new(),
            is_searching: false,
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        // 顶部搜索栏
        TopBottomPanel::top("search_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("搜索: ");
                let response = ui.text_edit_singleline(&mut self.search_query);

                if ui.button("🔍 搜索").clicked() ||
                    (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    self.perform_search();
                }

                if ui.button("📁 选择目录索引").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        // 异步索引目录
                        self.index_directory(path);
                    }
                }
            });
        });

        // 中央搜索结果区域
        CentralPanel::default().show(ctx, |ui| {
            if self.is_searching {
                ui.spinner();
                ui.label("搜索中...");
            } else {
                ScrollArea::vertical().show(ui, |ui| {
                    for result in &self.search_results {
                        ui.group(|ui| {
                            ui.heading(&result.title);
                            ui.label(format!("路径: {}", result.path.display()));
                            ui.label(format!("评分: {:.2}", result.score));
                            ui.separator();
                            ui.label(&result.content);

                            if ui.button("📖 打开文件").clicked() {
                                self.open_file(&result.path);
                            }
                        });
                        ui.separator();
                    }
                });
            }
        });
    }

    fn perform_search(&mut self) {
        if self.search_query.trim().is_empty() {
            return;
        }

        self.is_searching = true;
        match self.search_engine.search(&self.search_query, 20) {
            Ok(results) => {
                self.search_results = results;
            }
            Err(e) => {
                tracing::error!("搜索失败: {}", e);
            }
        }
        self.is_searching = false;
    }

    fn index_directory(&self, path: std::path::PathBuf) {
        // 这里需要异步处理，实际实现中可以使用消息传递
        tracing::info!("开始索引目录: {:?}", path);
    }

    fn open_file(&self, path: &std::path::Path) {
        if let Err(e) = opener::open(path) {
            tracing::error!("无法打开文件: {}", e);
        }
    }
}