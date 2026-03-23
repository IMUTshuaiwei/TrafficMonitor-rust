// src/utils/mod.rs

use chrono::{Local, Datelike};

pub fn today_str() -> String {
    let n = Local::now();
    format!("{:04}-{:02}-{:02}", n.year(), n.month(), n.day())
}

pub fn clamp_f32(val: f32, min: f32, max: f32) -> f32 {
    val.max(min).min(max)
}

pub fn lerp_color(c1: (u8,u8,u8,u8), c2: (u8,u8,u8,u8), t: f32) -> (u8,u8,u8,u8) {
    let t = clamp_f32(t, 0.0, 1.0);
    let l = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t) as u8;
    (l(c1.0,c2.0), l(c1.1,c2.1), l(c1.2,c2.2), l(c1.3,c2.3))
}

/// 加载 CJK 字体并注入 egui FontDefinitions
/// 优先级：微软雅黑 → 黑体 → 宋体 → 系统默认
pub fn load_cjk_font(fonts: &mut egui::FontDefinitions) {
    // 候选字体路径（Windows）
    let candidates = [
        r"C:\Windows\Fonts\msyh.ttc",   // 微软雅黑
        r"C:\Windows\Fonts\msyhbd.ttc",
        r"C:\Windows\Fonts\simhei.ttf", // 黑体
        r"C:\Windows\Fonts\simsun.ttc", // 宋体
        r"C:\Windows\Fonts\simkai.ttf", // 楷体
    ];

    for path in &candidates {
        if let Ok(data) = std::fs::read(path) {
            log::info!("Loaded CJK font: {path}");
            fonts.font_data.insert(
                "cjk_font".to_owned(),
                egui::FontData::from_owned(data),
            );
            // 插入到所有字体族的最后（fallback），这样只在 ASCII 找不到字形时才用它
            for family in fonts.families.values_mut() {
                family.push("cjk_font".to_owned());
            }
            return;
        }
    }

    // Linux / macOS fallback
    let linux_candidates = [
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/System/Library/Fonts/PingFang.ttc",
    ];
    for path in &linux_candidates {
        if let Ok(data) = std::fs::read(path) {
            log::info!("Loaded CJK font: {path}");
            fonts.font_data.insert("cjk_font".to_owned(), egui::FontData::from_owned(data));
            for family in fonts.families.values_mut() { family.push("cjk_font".to_owned()); }
            return;
        }
    }

    log::warn!("No CJK font found; Chinese characters may not render.");
}
