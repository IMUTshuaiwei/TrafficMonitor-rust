// src/monitor/gpu.rs  –  GPU 监控（NVIDIA via NVML，其他 GPU 优雅降级）

#[derive(Debug, Clone, Default)]
pub struct GpuStats {
    pub available: bool,
    pub name: String,
    pub usage_percent: f32,     // GPU 核心占用 %
    pub temperature: f32,       // GPU 温度 °C
    pub vram_used_mb: u64,      // 显存已用 MB
    pub vram_total_mb: u64,     // 显存总量 MB
    pub vram_percent: f32,      // 显存占用 %
}

// ─── Windows NVML 实现 ───────────────────────────────────────────────────────
#[cfg(target_os = "windows")]
pub struct GpuMonitor {
    inner: Option<NvmlMonitor>,
    pub stats: GpuStats,
}

#[cfg(target_os = "windows")]
struct NvmlMonitor {
    nvml: nvml_wrapper::Nvml,
}

#[cfg(target_os = "windows")]
impl GpuMonitor {
    pub fn new() -> Self {
        let inner = nvml_wrapper::Nvml::init().ok().map(|n| NvmlMonitor { nvml: n });
        if inner.is_none() {
            log::warn!("NVML 初始化失败，GPU 监控不可用（非 NVIDIA GPU 或驱动未安装）");
        }
        Self { inner, stats: GpuStats::default() }
    }

    pub fn refresh(&mut self) {
        let Some(m) = &self.inner else {
            self.stats.available = false;
            return;
        };
        let Ok(device) = m.nvml.device_by_index(0) else {
            self.stats.available = false;
            return;
        };

        self.stats.available = true;

        // GPU 名称
        self.stats.name = device.name().unwrap_or_else(|_| "GPU".to_string());

        // 核心占用
        if let Ok(util) = device.utilization_rates() {
            self.stats.usage_percent = util.gpu as f32;
        }

        // 温度
        if let Ok(temp) = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu) {
            self.stats.temperature = temp as f32;
        }

        // 显存
        if let Ok(mem) = device.memory_info() {
            const MB: u64 = 1024 * 1024;
            self.stats.vram_used_mb  = mem.used  / MB;
            self.stats.vram_total_mb = mem.total / MB;
            self.stats.vram_percent  = if mem.total > 0 {
                mem.used as f32 / mem.total as f32 * 100.0
            } else { 0.0 };
        }
    }
}

// ─── 非 Windows 空实现 ───────────────────────────────────────────────────────
#[cfg(not(target_os = "windows"))]
pub struct GpuMonitor { pub stats: GpuStats }

#[cfg(not(target_os = "windows"))]
impl GpuMonitor {
    pub fn new() -> Self { Self { stats: GpuStats::default() } }
    pub fn refresh(&mut self) { self.stats.available = false; }
}
