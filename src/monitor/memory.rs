// src/monitor/memory.rs
use sysinfo::System;

pub struct MemoryMonitor {
    system: System,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_percent: f32,
    history: Vec<f32>,
    history_max: usize,
}

impl MemoryMonitor {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_memory();
        let total = system.total_memory();
        let used = system.used_memory();
        let available = system.available_memory();
        let pct = if total > 0 { used as f32 / total as f32 * 100.0 } else { 0.0 };
        Self {
            system,
            total_bytes: total,
            used_bytes: used,
            available_bytes: available,
            usage_percent: pct,
            swap_total: 0,
            swap_used: 0,
            swap_percent: 0.0,
            history: Vec::new(),
            history_max: 60,
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_memory();
        self.total_bytes = self.system.total_memory();
        self.used_bytes = self.system.used_memory();
        self.available_bytes = self.system.available_memory();
        self.usage_percent = if self.total_bytes > 0 {
            (self.used_bytes as f32 / self.total_bytes as f32 * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };
        self.swap_total = self.system.total_swap();
        self.swap_used = self.system.used_swap();
        self.swap_percent = if self.swap_total > 0 {
            self.swap_used as f32 / self.swap_total as f32 * 100.0
        } else {
            0.0
        };
        self.history.push(self.usage_percent);
        if self.history.len() > self.history_max {
            self.history.remove(0);
        }
    }

    pub fn history(&self) -> &[f32] {
        &self.history
    }
}

pub fn format_memory(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else {
        format!("{:.0} MB", bytes as f64 / MB as f64)
    }
}
