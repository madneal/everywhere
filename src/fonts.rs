use egui::{Context, FontData, FontDefinitions, FontFamily};

pub fn install_cjk_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    let candidates: &[(&str, u32)] = if cfg!(target_os = "macos") {
        &[
            ("/System/Library/Fonts/Supplemental/Arial Unicode.ttf", 0),
            ("/Library/Fonts/Arial Unicode.ttf", 0),
            ("/System/Library/Fonts/PingFang.ttc", 4),
            ("/System/Library/Fonts/PingFang.ttc", 2),
            ("/System/Library/Fonts/PingFang.ttc", 0),
            ("/System/Library/Fonts/Hiragino Sans GB.ttc", 0),
            ("/System/Library/Fonts/STHeiti Medium.ttc", 0),
        ]
    } else if cfg!(target_os = "windows") {
        &[
            ("C:/Windows/Fonts/msyh.ttc", 0),
            ("C:/Windows/Fonts/msyhbd.ttc", 0),
            ("C:/Windows/Fonts/simhei.ttf", 0),
        ]
    } else {
        &[
            ("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.otf", 0),
            ("/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc", 0),
        ]
    };

    for &(path, index) in candidates {
        let Ok(bytes) = std::fs::read(path) else {
            continue;
        };
        let mut data = FontData::from_owned(bytes);
        data.index = index;
        fonts.font_data.insert("cjk-ui".to_string(), data);
        if let Some(v) = fonts.families.get_mut(&FontFamily::Proportional) {
            v.insert(0, "cjk-ui".to_string());
        }
        if let Some(v) = fonts.families.get_mut(&FontFamily::Monospace) {
            v.insert(0, "cjk-ui".to_string());
        }
        ctx.set_fonts(fonts);
        tracing::info!("egui CJK font from {} (face {})", path, index);
        return;
    }
    tracing::warn!("no CJK system font; non-Latin query text may show as boxes");
}
