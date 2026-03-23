// src/monitor/network.rs
use sysinfo::Networks;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    pub upload_speed: f64,
    pub download_speed: f64,
    pub total_upload: u64,
    pub total_download: u64,
    pub adapter_name: String,
}

pub struct NetworkMonitor {
    networks: Networks,
    prev_rx: HashMap<String, u64>,
    prev_tx: HashMap<String, u64>,
    selected_adapter: Option<String>,
    pub stats: NetworkStats,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();
        let mut prev_rx = HashMap::new();
        let mut prev_tx = HashMap::new();
        for (name, data) in networks.iter() {
            prev_rx.insert(name.clone(), data.total_received());
            prev_tx.insert(name.clone(), data.total_transmitted());
        }
        Self {
            networks,
            prev_rx,
            prev_tx,
            selected_adapter: None,
            stats: NetworkStats::default(),
        }
    }

    pub fn set_adapter(&mut self, name: Option<String>) {
        self.selected_adapter = name;
    }

    pub fn get_adapter_names(&self) -> Vec<String> {
        self.networks.iter().map(|(n, _)| n.clone()).collect()
    }

    pub fn refresh(&mut self, elapsed_secs: f64) {
        self.networks.refresh();

        let mut total_rx: u64 = 0;
        let mut total_tx: u64 = 0;
        let mut delta_rx: u64 = 0;
        let mut delta_tx: u64 = 0;
        let mut adapter_used = String::from("All");

        for (name, data) in self.networks.iter() {
            let should_include = match &self.selected_adapter {
                Some(sel) => sel == name,
                None => true,
            };
            if !should_include {
                continue;
            }

            let cur_rx = data.total_received();
            let cur_tx = data.total_transmitted();
            let prev_rx = self.prev_rx.get(name).copied().unwrap_or(cur_rx);
            let prev_tx = self.prev_tx.get(name).copied().unwrap_or(cur_tx);

            delta_rx += cur_rx.saturating_sub(prev_rx);
            delta_tx += cur_tx.saturating_sub(prev_tx);
            total_rx += cur_rx;
            total_tx += cur_tx;

            self.prev_rx.insert(name.clone(), cur_rx);
            self.prev_tx.insert(name.clone(), cur_tx);

            if let Some(sel) = &self.selected_adapter {
                if sel == name {
                    adapter_used = name.clone();
                }
            }
        }

        let safe_elapsed = if elapsed_secs > 0.0 { elapsed_secs } else { 1.0 };
        self.stats = NetworkStats {
            upload_speed: delta_tx as f64 / safe_elapsed,
            download_speed: delta_rx as f64 / safe_elapsed,
            total_upload: total_tx,
            total_download: total_rx,
            adapter_name: adapter_used,
        };
    }
}

pub fn format_speed(bytes_per_sec: f64) -> (String, String) {
    if bytes_per_sec >= 1024.0 * 1024.0 * 1024.0 {
        (format!("{:.2}", bytes_per_sec / (1024.0 * 1024.0 * 1024.0)), "GB/s".into())
    } else if bytes_per_sec >= 1024.0 * 1024.0 {
        (format!("{:.2}", bytes_per_sec / (1024.0 * 1024.0)), "MB/s".into())
    } else if bytes_per_sec >= 1024.0 {
        (format!("{:.1}", bytes_per_sec / 1024.0), "KB/s".into())
    } else {
        (format!("{:.0}", bytes_per_sec), "B/s".into())
    }
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 * 1024 {
        format!("{:.2} TB", bytes as f64 / (1024.0_f64.powi(4)))
    } else if bytes >= 1024 * 1024 * 1024 {
        format!("{:.2} GB", bytes as f64 / (1024.0_f64.powi(3)))
    } else if bytes >= 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0_f64.powi(2)))
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}
