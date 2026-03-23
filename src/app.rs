// src/app.rs

use std::time::{Duration, Instant};
use egui::{Color32, Pos2, Rect, Rounding, Sense, Stroke, Vec2};

use crate::config::{AppConfig, DisplayConfig};
use crate::monitor::SystemMonitor;
use crate::monitor::network::format_speed;
use crate::tray::{AppTray, TrayEvent};
use crate::ui::SettingsDialog;
use crate::utils::today_str;

const HISTORY_LEN: usize = 60;

pub struct SpeedHistory {
    pub up: Vec<f64>, pub down: Vec<f64>, pub max: f64,
}
impl SpeedHistory {
    fn new() -> Self { Self { up: vec![0.0; HISTORY_LEN], down: vec![0.0; HISTORY_LEN], max: 1024.0 } }
    fn push(&mut self, up: f64, down: f64) {
        self.up.push(up);   if self.up.len()   > HISTORY_LEN { self.up.remove(0); }
        self.down.push(down); if self.down.len() > HISTORY_LEN { self.down.remove(0); }
        let peak = self.up.iter().chain(self.down.iter()).cloned().fold(0.0_f64, f64::max);
        self.max = if peak < 1024.0 { 1024.0 } else { peak * 1.2 };
    }
}

// 根据显示配置计算窗口尺寸（左右两列布局）
fn compute_window_size(d: &DisplayConfig, font_size: f32) -> Vec2 {
    let row_h = font_size + 5.0;
    let bar_h = 4.0;
    let gap   = 2.0;
    let pad_x = 10.0_f32;
    let pad_y = 10.0_f32;

    // 左列：上传 + 下载 + 曲线图
    let mut left_h = pad_y;
    if d.show_upload   { left_h += row_h + gap; }
    if d.show_download { left_h += row_h + gap; }
    if d.show_net_graph { left_h += 42.0 + gap; }

    // 右列：CPU/MEM/GPU 各行
    let mut right_h = pad_y;
    if d.show_cpu    { right_h += row_h + gap; if d.show_cpu_bar { right_h += bar_h + gap; } }
    if d.show_cpu_temp { right_h += row_h + gap; }
    if d.show_memory { right_h += row_h + gap; if d.show_mem_bar { right_h += bar_h + gap; } }
    if d.show_gpu    { right_h += row_h + gap; if d.show_gpu_bar { right_h += bar_h + gap; } }
    if d.show_gpu_temp { right_h += row_h + gap; }
    if d.show_vram   { right_h += row_h + gap; }

    let h = left_h.max(right_h) + pad_y;
    // 左列宽：网速最长 "999.99 MB/s" ~ font*8.5
    // 右列宽：数值 + 标签 ~ font*9
    let col_w = font_size * 9.0 + pad_x;
    Vec2::new(col_w * 2.0 + 6.0, h)   // 6px 分隔线
}

pub struct TrafficMonitorApp {
    cfg: AppConfig,
    monitor: SystemMonitor,
    last_refresh: Instant,
    speed_history: SpeedHistory,
    settings_dlg: SettingsDialog,
    tray: Option<AppTray>,
    dragging: bool,
    drag_cursor_origin: Option<Pos2>,
    drag_window_origin: Option<Pos2>,
    today_str: String,
    last_sent_size: Vec2,
    last_penetrate: Option<bool>,
}

impl TrafficMonitorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let cfg = AppConfig::load();

        let mut fonts = egui::FontDefinitions::default();
        crate::utils::load_cjk_font(&mut fonts);
        cc.egui_ctx.set_fonts(fonts);

        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals.window_shadow = egui::epaint::Shadow::NONE;
        style.visuals.popup_shadow  = egui::epaint::Shadow::NONE;
        style.spacing.item_spacing  = Vec2::new(3.0, 2.0);
        cc.egui_ctx.set_style(style);

        let monitor = SystemMonitor::new();
        Self {
            cfg, monitor,
            last_refresh: Instant::now() - Duration::from_secs(2),
            speed_history: SpeedHistory::new(),
            settings_dlg: SettingsDialog::default(),
            tray: AppTray::new(),
            dragging: false,
            drag_cursor_origin: None,
            drag_window_origin: None,
            today_str: today_str(),
            last_sent_size: Vec2::ZERO,
            last_penetrate: None,
        }
    }

    fn maybe_refresh(&mut self) {
        if self.last_refresh.elapsed() < Duration::from_millis(self.cfg.general.refresh_interval_ms) { return; }
        self.last_refresh = Instant::now();
        self.monitor.network.set_adapter(self.cfg.general.selected_adapter.clone());
        self.monitor.refresh();
        let (us, ds) = { let n = &self.monitor.network.stats; (n.upload_speed, n.download_speed) };
        self.speed_history.push(us, ds);
    }

    fn maybe_resize(&mut self, ctx: &egui::Context) {
        let desired = compute_window_size(&self.cfg.display, self.cfg.active_skin().font_size);
        if (desired.x - self.last_sent_size.x).abs() > 0.5
        || (desired.y - self.last_sent_size.y).abs() > 0.5 {
            self.last_sent_size = desired;
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(desired));
        }
    }

    fn apply_mouse_penetrate(&mut self, ctx: &egui::Context) {
        let want = self.cfg.general.mouse_penetrate;
        if self.last_penetrate != Some(want) {
            self.last_penetrate = Some(want);
            ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(want));
        }
    }

    fn poll_tray(&mut self, ctx: &egui::Context) {
        let ev = self.tray.as_ref().and_then(|t| t.poll());
        match ev {
            Some(TrayEvent::ShowOptions) => self.settings_dlg.open = true,
            Some(TrayEvent::Exit) => {
                self.cfg.save();
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            None => {}
        }
    }

    fn draw(&mut self, ctx: &egui::Context) {
        let skin      = self.cfg.active_skin().clone();
        let display   = self.cfg.display.clone();
        let net       = self.monitor.network.stats.clone();
        let cpu_u     = self.monitor.cpu.usage;
        let cpu_temp  = self.monitor.cpu.temperature;
        let mem_u     = self.monitor.memory.usage_percent;
        let gpu       = self.monitor.gpu.stats.clone();
        let fs        = skin.font_size;
        let ss        = (fs - 1.5).max(7.0);

        // ── 透明度：只作用于背景和边框，文字颜色完全不受影响 ─────────────
        // opacity=0 → 背景完全透明，只有文字可见
        // opacity=1 → 背景完全不透明
        let opacity = skin.window_opacity.clamp(0.0, 1.0);
        let fade = |c: Color32| -> Color32 {
            Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (c.a() as f32 * opacity) as u8)
        };
        let bg           = fade(skin.background_color.to_egui());
        let border_color = fade(skin.border_color.to_egui());

        // 文字和数据颜色：不受透明度影响
        let text_c = skin.text_color.to_egui();
        let up_c   = skin.upload_color.to_egui();
        let dn_c   = skin.download_color.to_egui();
        let cpu_c  = skin.cpu_bar_color.to_egui();
        let mem_c  = skin.mem_bar_color.to_egui();
        let gpu_c  = skin.gpu_bar_color.to_egui();
        let rounding = Rounding::same(skin.corner_radius);

        // ── 拖拽 ─────────────────────────────────────────────────────────
        if !self.cfg.general.window_locked && !self.cfg.general.mouse_penetrate {
            let pressed  = ctx.input(|i| i.pointer.primary_pressed());
            let down     = ctx.input(|i| i.pointer.primary_down());
            let released = ctx.input(|i| i.pointer.primary_released());
            let win_tl   = ctx.input(|i| i.viewport().outer_rect.map(|r| r.min));
            let local    = ctx.input(|i| i.pointer.interact_pos());

            if pressed {
                if let (Some(wtl), Some(lp)) = (win_tl, local) {
                    self.dragging = true;
                    self.drag_cursor_origin = Some(wtl + lp.to_vec2());
                    self.drag_window_origin = Some(wtl);
                }
            }
            if released { self.dragging = false; }
            if self.dragging && down {
                if let (Some(c0), Some(w0), Some(wtl), Some(lp)) =
                    (self.drag_cursor_origin, self.drag_window_origin, win_tl, local)
                {
                    let new_pos = w0 + ((wtl + lp.to_vec2()) - c0);
                    if (new_pos.x - self.cfg.general.window_position.x).abs() > 0.5
                    || (new_pos.y - self.cfg.general.window_position.y).abs() > 0.5 {
                        self.cfg.general.window_position.x = new_pos.x;
                        self.cfg.general.window_position.y = new_pos.y;
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(new_pos));
                    }
                }
            }
        }

        // ── 主面板 ───────────────────────────────────────────────────────
        let frame = egui::Frame {
            fill: bg,
            inner_margin: egui::Margin::symmetric(8.0, 6.0),
            outer_margin: egui::Margin::ZERO,
            rounding,
            stroke: Stroke::new(1.0, border_color),
            shadow: egui::epaint::Shadow::NONE,
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(4.0, 2.0);

                // ── 左列：网速 ─────────────────────────────────────────
                ui.vertical(|ui| {
                    let col_w = fs * 9.0;
                    ui.set_width(col_w);
                    let row_h = fs + 4.0;

                    if display.show_upload {
                        let (v, u) = format_speed(net.upload_speed);
                        speed_row(ui, true,  &v, if display.show_units { &u } else { "" }, up_c, fs, ss, row_h);
                    }
                    if display.show_download {
                        let (v, u) = format_speed(net.download_speed);
                        speed_row(ui, false, &v, if display.show_units { &u } else { "" }, dn_c, fs, ss, row_h);
                    }
                    if display.show_net_graph {
                        draw_net_graph(ui, &self.speed_history, up_c, dn_c);
                    }
                });

                // 竖向分隔线
                let sep_color = border_color.gamma_multiply(1.5);
                let rect = ui.available_rect_before_wrap();
                ui.painter().line_segment(
                    [Pos2::new(rect.left() + 2.0, rect.top()), Pos2::new(rect.left() + 2.0, rect.bottom())],
                    Stroke::new(1.0, sep_color),
                );
                ui.add_space(6.0);

                // ── 右列：系统资源 ──────────────────────────────────────
                ui.vertical(|ui| {
                    let col_w = fs * 9.0;
                    ui.set_width(col_w);
                    let row_h = fs + 4.0;

                    if display.show_cpu {
                        let s = pct_str(cpu_u, display.show_percent_sign);
                        stat_row(ui, "CPU", &s, cpu_u/100.0, cpu_c, text_c, fs, ss, display.show_cpu_bar);
                    }
                    if display.show_cpu_temp {
                        if cpu_temp >= 0.0 {
                            temp_row(ui, "CPU温度", cpu_temp, cpu_c, text_c, fs, ss, row_h);
                        } else {
                            label_row(ui, "CPU温度", "N/A", text_c.gamma_multiply(0.5), text_c, fs, ss, row_h);
                        }
                    }
                    if display.show_memory {
                        let s = pct_str(self.monitor.memory.usage_percent, display.show_percent_sign);
                        stat_row(ui, "内存", &s, mem_u/100.0, mem_c, text_c, fs, ss, display.show_mem_bar);
                    }
                    if display.show_gpu {
                        if gpu.available {
                            let s = pct_str(gpu.usage_percent, display.show_percent_sign);
                            stat_row(ui, "GPU", &s, gpu.usage_percent/100.0, gpu_c, text_c, fs, ss, display.show_gpu_bar);
                        } else {
                            label_row(ui, "GPU", "N/A", text_c.gamma_multiply(0.5), text_c, fs, ss, row_h);
                        }
                    }
                    if display.show_gpu_temp && gpu.available {
                        temp_row(ui, "GPU温度", gpu.temperature, gpu_c, text_c, fs, ss, row_h);
                    }
                    if display.show_vram && gpu.available {
                        let s = if gpu.vram_total_mb > 0 {
                            format!("{}/{} MB", gpu.vram_used_mb, gpu.vram_total_mb)
                        } else { "N/A".into() };
                        label_row(ui, "显存", &s, gpu_c, text_c, fs, ss, row_h);
                    }
                });
            });
        });
    }
}

// ─── 行渲染工具函数 ──────────────────────────────────────────────────────────

fn pct_str(v: f32, show_sign: bool) -> String {
    if show_sign { format!("{:.1}%", v) } else { format!("{:.1}", v) }
}

/// 网速行：三角图标 + 数值 + 单位
fn speed_row(ui: &mut egui::Ui, is_up: bool, val: &str, unit: &str,
             color: Color32, fs: f32, ss: f32, row_h: f32) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 3.0;
        let iw = fs * 0.68;
        let (ir, _) = ui.allocate_exact_size(Vec2::new(iw, row_h), Sense::hover());
        let p = ui.painter();
        let (cx, cy, h, w2) = (ir.center().x, ir.center().y, iw*0.82, iw*0.50);
        if is_up {
            p.add(egui::epaint::PathShape::convex_polygon(
                vec![Pos2::new(cx, cy-h*0.5), Pos2::new(cx+w2, cy+h*0.5), Pos2::new(cx-w2, cy+h*0.5)],
                color, Stroke::NONE));
        } else {
            p.add(egui::epaint::PathShape::convex_polygon(
                vec![Pos2::new(cx, cy+h*0.5), Pos2::new(cx+w2, cy-h*0.5), Pos2::new(cx-w2, cy-h*0.5)],
                color, Stroke::NONE));
        }
        ui.colored_label(color, egui::RichText::new(val).size(fs).strong());
        if !unit.is_empty() {
            ui.colored_label(color.gamma_multiply(0.70), egui::RichText::new(unit).size(ss));
        }
    });
}

/// 带进度条的百分比行
fn stat_row(ui: &mut egui::Ui, label: &str, pct: &str, frac: f32,
            bar_c: Color32, text_c: Color32, fs: f32, ss: f32, show_bar: bool) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 3.0;
        ui.colored_label(text_c.gamma_multiply(0.55), egui::RichText::new(label).size(ss));
        ui.colored_label(text_c, egui::RichText::new(pct).size(fs).strong());
    });
    if show_bar {
        let (resp, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), 3.0), Sense::hover());
        let r = resp.rect;
        painter.rect_filled(r, Rounding::same(1.5), bar_c.gamma_multiply(0.2));
        let fw = r.width() * frac.clamp(0.0, 1.0);
        if fw > 0.0 {
            painter.rect_filled(Rect::from_min_size(r.min, Vec2::new(fw, r.height())), Rounding::same(1.5), bar_c);
        }
    }
}

/// 温度行：带 °C 单位，颜色随温度渐变（绿→黄→红）
fn temp_row(ui: &mut egui::Ui, label: &str, temp: f32,
            accent: Color32, text_c: Color32, fs: f32, ss: f32, row_h: f32) {
    let temp_color = if temp < 50.0 { Color32::from_rgb(80, 210, 80) }
                     else if temp < 70.0 { Color32::from_rgb(255, 200, 50) }
                     else if temp < 85.0 { Color32::from_rgb(255, 120, 30) }
                     else { Color32::from_rgb(255, 50, 50) };
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 3.0;
        ui.colored_label(text_c.gamma_multiply(0.55), egui::RichText::new(label).size(ss));
        ui.colored_label(temp_color, egui::RichText::new(format!("{:.0}°C", temp)).size(fs).strong());
    });
}

/// 普通标签 + 数值行（显存等）
fn label_row(ui: &mut egui::Ui, label: &str, val: &str,
             val_color: Color32, text_c: Color32, fs: f32, ss: f32, _row_h: f32) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 3.0;
        ui.colored_label(text_c.gamma_multiply(0.55), egui::RichText::new(label).size(ss));
        ui.colored_label(val_color, egui::RichText::new(val).size(fs).strong());
    });
}

fn draw_net_graph(ui: &mut egui::Ui, history: &SpeedHistory, up_c: Color32, dn_c: Color32) {
    let (resp, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), 40.0), Sense::hover());
    let r = resp.rect;
    painter.rect_filled(r, Rounding::same(2.0), Color32::from_black_alpha(50));
    let n = history.up.len();
    if n < 2 || history.max <= 0.0 { return; }
    let to_y = |v: f64| r.bottom() - (v / history.max).min(1.0) as f32 * r.height();
    let to_x = |i: usize| r.left() + i as f32 / (HISTORY_LEN - 1) as f32 * r.width();
    for i in 1..n {
        let (x0, x1) = (to_x(i-1), to_x(i));
        painter.line_segment([Pos2::new(x0, to_y(history.down[i-1])), Pos2::new(x1, to_y(history.down[i]))], Stroke::new(1.5, dn_c));
        painter.line_segment([Pos2::new(x0, to_y(history.up[i-1])),   Pos2::new(x1, to_y(history.up[i]))],   Stroke::new(1.5, up_c));
    }
}

// ─── eframe::App ─────────────────────────────────────────────────────────────

impl eframe::App for TrafficMonitorApp {
    fn clear_color(&self, _: &egui::Visuals) -> [f32; 4] { [0.0, 0.0, 0.0, 0.0] }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.maybe_refresh();
        self.maybe_resize(ctx);
        self.apply_mouse_penetrate(ctx);
        self.poll_tray(ctx);
        self.draw(ctx);
        self.settings_dlg.show(ctx, &mut self.cfg, &self.monitor.network.get_adapter_names());
        ctx.request_repaint_after(Duration::from_millis(self.cfg.general.refresh_interval_ms));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) { self.cfg.save(); }
}
