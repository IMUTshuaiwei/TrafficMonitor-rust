pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod network;

pub use cpu::CpuMonitor;
pub use gpu::GpuMonitor;
pub use memory::MemoryMonitor;
pub use network::NetworkMonitor;

use std::time::{Duration, Instant};

pub struct SystemMonitor {
    pub cpu: CpuMonitor,
    pub memory: MemoryMonitor,
    pub network: NetworkMonitor,
    pub gpu: GpuMonitor,
    last_refresh: Instant,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            cpu: CpuMonitor::new(),
            memory: MemoryMonitor::new(),
            network: NetworkMonitor::new(),
            gpu: GpuMonitor::new(),
            last_refresh: Instant::now(),
        }
    }

    pub fn refresh(&mut self) -> f64 {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refresh).as_secs_f64();
        self.last_refresh = now;
        self.cpu.refresh();
        self.memory.refresh();
        self.network.refresh(elapsed);
        self.gpu.refresh();
        elapsed
    }
}
