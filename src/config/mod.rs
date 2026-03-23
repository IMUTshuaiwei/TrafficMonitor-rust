use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Color32Cfg { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }
impl Color32Cfg {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }
    pub fn to_egui(&self) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(self.r, self.g, self.b, self.a)
    }
}
impl From<egui::Color32> for Color32Cfg {
    fn from(c: egui::Color32) -> Self { Self::new(c.r(), c.g(), c.b(), c.a()) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPos { pub x: f32, pub y: f32 }
impl Default for WindowPos { fn default() -> Self { Self { x: 100.0, y: 100.0 } } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinConfig {
    pub name: String,
    pub background_color: Color32Cfg,
    pub text_color: Color32Cfg,
    pub upload_color: Color32Cfg,
    pub download_color: Color32Cfg,
    pub cpu_bar_color: Color32Cfg,
    pub mem_bar_color: Color32Cfg,
    pub gpu_bar_color: Color32Cfg,
    pub border_color: Color32Cfg,
    pub font_size: f32,
    pub window_opacity: f32,   // 0.0 = 全透明仅剩文字，1.0 = 不透明
    pub corner_radius: f32,
}

pub fn builtin_skins() -> Vec<SkinConfig> {
    vec![
        SkinConfig {
            name: "深色".into(),
            background_color: Color32Cfg::new(18, 18, 24, 255),
            text_color:     Color32Cfg::new(220, 220, 220, 255),
            upload_color:   Color32Cfg::new(80,  210, 100, 255),
            download_color: Color32Cfg::new(80,  150, 255, 255),
            cpu_bar_color:  Color32Cfg::new(255, 175, 50,  255),
            mem_bar_color:  Color32Cfg::new(200, 100, 255, 255),
            gpu_bar_color:  Color32Cfg::new(0,   220, 200, 255),
            border_color:   Color32Cfg::new(60,  60,  80,  255),
            font_size: 12.0, window_opacity: 0.85, corner_radius: 8.0,
        },
        SkinConfig {
            name: "浅色".into(),
            background_color: Color32Cfg::new(240, 240, 245, 255),
            text_color:     Color32Cfg::new(30,  30,  30,  255),
            upload_color:   Color32Cfg::new(0,   150, 60,  255),
            download_color: Color32Cfg::new(0,   80,  200, 255),
            cpu_bar_color:  Color32Cfg::new(220, 120, 0,   255),
            mem_bar_color:  Color32Cfg::new(150, 0,   200, 255),
            gpu_bar_color:  Color32Cfg::new(0,   180, 160, 255),
            border_color:   Color32Cfg::new(180, 180, 180, 255),
            font_size: 12.0, window_opacity: 0.90, corner_radius: 8.0,
        },
        SkinConfig {
            name: "绿色矩阵".into(),
            background_color: Color32Cfg::new(0,   18,  0,   255),
            text_color:     Color32Cfg::new(0,   255, 80,  255),
            upload_color:   Color32Cfg::new(0,   200, 100, 255),
            download_color: Color32Cfg::new(0,   255, 180, 255),
            cpu_bar_color:  Color32Cfg::new(50,  255, 50,  255),
            mem_bar_color:  Color32Cfg::new(0,   200, 100, 255),
            gpu_bar_color:  Color32Cfg::new(100, 255, 220, 255),
            border_color:   Color32Cfg::new(0,   100, 0,   255),
            font_size: 12.0, window_opacity: 0.82, corner_radius: 6.0,
        },
        SkinConfig {
            name: "蓝色霓虹".into(),
            background_color: Color32Cfg::new(5,   8,   30,  255),
            text_color:     Color32Cfg::new(100, 180, 255, 255),
            upload_color:   Color32Cfg::new(0,   220, 255, 255),
            download_color: Color32Cfg::new(80,  120, 255, 255),
            cpu_bar_color:  Color32Cfg::new(0,   200, 255, 255),
            mem_bar_color:  Color32Cfg::new(150, 80,  255, 255),
            gpu_bar_color:  Color32Cfg::new(0,   255, 200, 255),
            border_color:   Color32Cfg::new(0,   80,  160, 255),
            font_size: 12.0, window_opacity: 0.82, corner_radius: 8.0,
        },
        SkinConfig {
            name: "极简白".into(),
            background_color: Color32Cfg::new(255, 255, 255, 255),
            text_color:     Color32Cfg::new(40,  40,  40,  255),
            upload_color:   Color32Cfg::new(0,   160, 80,  255),
            download_color: Color32Cfg::new(0,   100, 220, 255),
            cpu_bar_color:  Color32Cfg::new(255, 140, 0,   255),
            mem_bar_color:  Color32Cfg::new(180, 0,   200, 255),
            gpu_bar_color:  Color32Cfg::new(0,   190, 170, 255),
            border_color:   Color32Cfg::new(200, 200, 200, 255),
            font_size: 12.0, window_opacity: 0.78, corner_radius: 12.0,
        },
    ]
}

impl Default for SkinConfig { fn default() -> Self { builtin_skins().remove(0) } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub show_upload:   bool,
    pub show_download: bool,
    pub show_cpu:      bool,
    pub show_cpu_temp: bool,
    pub show_memory:   bool,
    pub show_cpu_bar:  bool,
    pub show_mem_bar:  bool,
    pub show_gpu:      bool,
    pub show_gpu_temp: bool,
    pub show_vram:     bool,
    pub show_gpu_bar:  bool,
    pub show_net_graph:    bool,
    pub show_units:        bool,
    pub show_percent_sign: bool,
}
impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_upload: true, show_download: true,
            show_cpu: true, show_cpu_temp: true,
            show_memory: true, show_cpu_bar: true, show_mem_bar: true,
            show_gpu: true, show_gpu_temp: true, show_vram: true, show_gpu_bar: true,
            show_net_graph: false,
            show_units: true, show_percent_sign: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub always_on_top:     bool,
    pub mouse_penetrate:   bool,
    pub window_locked:     bool,
    pub refresh_interval_ms: u64,
    pub selected_adapter:  Option<String>,
    pub selected_skin_index: usize,
    pub window_position:   WindowPos,
}
impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            always_on_top: true, mouse_penetrate: false, window_locked: false,
            refresh_interval_ms: 1000,
            selected_adapter: None, selected_skin_index: 0,
            window_position: WindowPos::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub display: DisplayConfig,
    pub skins: Vec<SkinConfig>,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self { general: GeneralConfig::default(), display: DisplayConfig::default(), skins: builtin_skins() }
    }
}
impl AppConfig {
    pub fn config_path() -> PathBuf {
        dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
            .join("TrafficMonitorRs").join("config.json")
    }
    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(cfg) = serde_json::from_str(&data) { return cfg; }
        }
        Self::default()
    }
    pub fn save(&self) {
        let path = Self::config_path();
        if let Some(p) = path.parent() { let _ = std::fs::create_dir_all(p); }
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, data);
        }
    }
    pub fn active_skin(&self) -> &SkinConfig {
        let idx = self.general.selected_skin_index.min(self.skins.len().saturating_sub(1));
        &self.skins[idx]
    }
}
