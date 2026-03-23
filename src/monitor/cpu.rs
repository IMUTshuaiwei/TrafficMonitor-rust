// src/monitor/cpu.rs
use sysinfo::{System, Components};

pub struct CpuMonitor {
    system: System,
    components: Components,
    pub usage: f32,
    pub temperature: f32,   // °C，-1.0 表示不可用
    pub frequency_mhz: u64,
    pub core_usages: Vec<f32>,
    history: Vec<f32>,
    history_max: usize,
}

impl CpuMonitor {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_cpu_usage();
        let components = Components::new_with_refreshed_list();
        Self {
            system, components,
            usage: 0.0, temperature: -1.0, frequency_mhz: 0,
            core_usages: Vec::new(),
            history: Vec::new(), history_max: 60,
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_cpu_usage();
        let cpus = self.system.cpus();
        if !cpus.is_empty() {
            let total: f32 = cpus.iter().map(|c| c.cpu_usage()).sum();
            self.usage = (total / cpus.len() as f32).clamp(0.0, 100.0);
            self.core_usages = cpus.iter().map(|c| c.cpu_usage()).collect();
            self.frequency_mhz = cpus.first().map(|c| c.frequency()).unwrap_or(0);
        }
        self.history.push(self.usage);
        if self.history.len() > self.history_max { self.history.remove(0); }

        // CPU 温度：从组件中找 "CPU" 相关条目
        self.components.refresh();
        self.temperature = -1.0;
        for comp in self.components.iter() {
            let label = comp.label().to_lowercase();
            if label.contains("cpu") || label.contains("package") || label.contains("tctl") {
                if let Some(t) = Some(comp.temperature()) {
                    if t > 0.0 { self.temperature = t; break; }
                }
            }
        }
    }

    pub fn history(&self) -> &[f32] { &self.history }
}
