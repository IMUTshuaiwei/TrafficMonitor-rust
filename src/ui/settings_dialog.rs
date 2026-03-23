use crate::config::AppConfig;
use egui::{Context, Ui, ComboBox, ViewportBuilder, ViewportId};

pub struct SettingsDialog { pub open: bool, tab: Tab }
#[derive(PartialEq, Eq, Clone, Copy)]
enum Tab { 常规, 显示, 皮肤, 网络, 关于 }
impl Default for SettingsDialog { fn default() -> Self { Self { open: false, tab: Tab::常规 } } }

impl SettingsDialog {
    pub fn show(&mut self, ctx: &Context, cfg: &mut AppConfig, adapters: &[String]) {
        if !self.open { return; }
        let mut open = true;
        ctx.show_viewport_immediate(
            ViewportId::from_hash_of("settings_wnd"),
            ViewportBuilder::default()
                .with_title("流量监控 – 选项设置")
                .with_inner_size([460.0, 380.0])
                .with_min_inner_size([360.0, 260.0])
                .with_resizable(true),
            |ctx, _| {
                if ctx.input(|i| i.viewport().close_requested()) { open = false; }
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.tab, Tab::常规, "⚙ 常规");
                        ui.selectable_value(&mut self.tab, Tab::显示, "👁 显示");
                        ui.selectable_value(&mut self.tab, Tab::皮肤, "🎨 皮肤");
                        ui.selectable_value(&mut self.tab, Tab::网络, "🌐 网络");
                        ui.selectable_value(&mut self.tab, Tab::关于, "ℹ 关于");
                    });
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        match self.tab {
                            Tab::常规 => tab_general(ui, cfg),
                            Tab::显示 => tab_display(ui, cfg),
                            Tab::皮肤 => tab_skin(ui, cfg),
                            Tab::网络 => tab_network(ui, cfg, adapters),
                            Tab::关于 => { tab_about(ui); return; }
                        }
                    });
                    if self.tab != Tab::关于 {
                        ui.separator();
                        ui.horizontal(|ui| {
                            if ui.button("💾 保存").clicked() { cfg.save(); open = false; }
                            if ui.button("关闭").clicked()    { open = false; }
                        });
                    }
                });
            },
        );
        self.open = open;
    }
}

fn tab_general(ui: &mut Ui, cfg: &mut AppConfig) {
    let g = &mut cfg.general;
    ui.heading("窗口");
    ui.checkbox(&mut g.always_on_top,   "始终置顶");
    ui.checkbox(&mut g.mouse_penetrate, "鼠标穿透（启用后右键悬浮窗无效，需关闭后操作）");
    ui.checkbox(&mut g.window_locked,   "锁定位置（禁止拖动）");
    ui.separator();
    ui.heading("刷新频率");
    ui.horizontal(|ui| {
        ui.label("间隔：");
        ui.add(egui::DragValue::new(&mut g.refresh_interval_ms)
            .speed(50).clamp_range(200u64..=5000u64).suffix(" ms"));
    });
}

fn tab_display(ui: &mut Ui, cfg: &mut AppConfig) {
    let d = &mut cfg.display;
    ui.heading("左列：网速");
    ui.checkbox(&mut d.show_upload,    "显示上传速度");
    ui.checkbox(&mut d.show_download,  "显示下载速度");
    ui.checkbox(&mut d.show_units,     "显示速度单位");
    ui.checkbox(&mut d.show_net_graph, "显示网速历史曲线");
    ui.separator();
    ui.heading("右列：CPU / 内存");
    ui.checkbox(&mut d.show_cpu,      "显示 CPU 占用");
    ui.checkbox(&mut d.show_cpu_bar,  "  └ 显示 CPU 进度条");
    ui.checkbox(&mut d.show_cpu_temp, "显示 CPU 温度");
    ui.checkbox(&mut d.show_memory,   "显示内存占用");
    ui.checkbox(&mut d.show_mem_bar,  "  └ 显示内存进度条");
    ui.separator();
    ui.heading("右列：GPU（需 NVIDIA + 驱动）");
    ui.checkbox(&mut d.show_gpu,      "显示显卡占用");
    ui.checkbox(&mut d.show_gpu_bar,  "  └ 显示显卡进度条");
    ui.checkbox(&mut d.show_gpu_temp, "显示显卡温度");
    ui.checkbox(&mut d.show_vram,     "显示显存占用（已用/总量 MB）");
    ui.separator();
    ui.checkbox(&mut d.show_percent_sign, "显示百分号（%）");
}

fn tab_skin(ui: &mut Ui, cfg: &mut AppConfig) {
    let names: Vec<String> = cfg.skins.iter().map(|s| s.name.clone()).collect();
    let mut sel = cfg.general.selected_skin_index;
    ComboBox::from_label("当前皮肤")
        .selected_text(names.get(sel).cloned().unwrap_or_default())
        .show_ui(ui, |ui| {
            for (i, n) in names.iter().enumerate() { ui.selectable_value(&mut sel, i, n); }
        });
    cfg.general.selected_skin_index = sel;
    ui.separator();

    let skin = &mut cfg.skins[sel];
    ui.heading(format!("调节「{}」", skin.name));

    ui.horizontal(|ui| {
        ui.label("透明度：");
        ui.add(egui::Slider::new(&mut skin.window_opacity, 0.0..=1.0).step_by(0.01));
        ui.label(format!("{:.0}%", skin.window_opacity * 100.0));
    });
    ui.label(egui::RichText::new("← 0% 仅显示文字，100% 完全不透明").size(10.0).color(egui::Color32::GRAY));

    ui.horizontal(|ui| {
        ui.label("圆角：");
        ui.add(egui::Slider::new(&mut skin.corner_radius, 0.0..=24.0).step_by(1.0).suffix(" px"));
    });
    ui.horizontal(|ui| {
        ui.label("字体：");
        ui.add(egui::Slider::new(&mut skin.font_size, 9.0..=18.0).step_by(0.5).suffix(" px"));
    });

    ui.separator();
    ui.label("颜色预览：");
    ui.horizontal(|ui| {
        color_chip(ui, "背景",  skin.background_color.to_egui());
        color_chip(ui, "上传",  skin.upload_color.to_egui());
        color_chip(ui, "下载",  skin.download_color.to_egui());
        color_chip(ui, "CPU",   skin.cpu_bar_color.to_egui());
        color_chip(ui, "内存",  skin.mem_bar_color.to_egui());
        color_chip(ui, "GPU",   skin.gpu_bar_color.to_egui());
    });
}

fn color_chip(ui: &mut Ui, label: &str, color: egui::Color32) {
    ui.vertical(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(18.0), egui::Sense::hover());
        ui.painter().rect_filled(rect, egui::Rounding::same(3.0), color);
        ui.painter().rect_stroke(rect, egui::Rounding::same(3.0), egui::Stroke::new(1.0, egui::Color32::from_white_alpha(60)));
        ui.label(egui::RichText::new(label).size(9.0));
    });
}

fn tab_network(ui: &mut Ui, cfg: &mut AppConfig, adapters: &[String]) {
    ui.heading("网络适配器");
    let cur = cfg.general.selected_adapter.clone().unwrap_or_else(|| "自动（合计所有）".into());
    ComboBox::from_label("网卡").selected_text(&cur).show_ui(ui, |ui| {
        if ui.selectable_label(cfg.general.selected_adapter.is_none(), "自动（合计所有）").clicked() {
            cfg.general.selected_adapter = None;
        }
        for n in adapters {
            if ui.selectable_label(cfg.general.selected_adapter.as_deref() == Some(n), n).clicked() {
                cfg.general.selected_adapter = Some(n.clone());
            }
        }
    });
    ui.separator();
    for n in adapters { ui.label(format!("  • {n}")); }
}

fn tab_about(ui: &mut Ui) {
    ui.add_space(16.0);
    ui.vertical_centered(|ui| {
        ui.heading("流量监控");
        ui.add_space(4.0);
        ui.label(egui::RichText::new("作者：lsw").size(14.0));
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(6.0);
        ui.label(egui::RichText::new("Rust · egui · sysinfo · nvml").size(10.0).color(egui::Color32::GRAY));
    });
}
