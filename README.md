# Traffic Monitor (Rust Edition)

A complete Rust rewrite of [TrafficMonitor](https://github.com/zhongyang219/TrafficMonitor) by zhongyang219.

## Features

- ✅ Real-time network upload/download speed monitoring
- ✅ CPU usage monitoring (with history)  
- ✅ Memory (RAM) usage monitoring (with history)
- ✅ Always-on-top borderless floating window
- ✅ Draggable window (click-and-drag to reposition)
- ✅ Network speed history graph
- ✅ CPU/Memory progress bars
- ✅ 4 built-in skins (Dark, Light, Green Matrix, Blue Neon)
- ✅ Custom skin editor with full color pickers
- ✅ Per-adapter or all-adapter network monitoring
- ✅ Settings persistence (JSON config in AppData)
- ✅ Historical daily/monthly traffic statistics
- ✅ Right-click context menu
- ✅ Memory usage alert notifications
- ✅ Configurable refresh interval
- ✅ Lock/unlock window position
- ✅ Horizontal or vertical layout

## Building

### Prerequisites

- Rust 1.75+ (install from https://rustup.rs)
- Windows 10/11 (primary target; also compiles on Linux/macOS)

### Build

```bash
# Debug build (fast compile, slower runtime)
cargo build

# Release build (optimized, ~1MB executable)
cargo build --release

# Run directly
cargo run
cargo run --release
```

The release binary will be at `target/release/traffic-monitor.exe`.

## Project Structure

```
traffic-monitor/
├── Cargo.toml              # Dependencies
├── build.rs                # Windows resource embedding
├── README.md               # This file
└── src/
    ├── main.rs             # Entry point, window configuration
    ├── app.rs              # Main app logic, UI rendering, eframe::App
    ├── monitor/
    │   ├── mod.rs          # SystemMonitor aggregator
    │   ├── network.rs      # Network speed monitoring (sysinfo)
    │   ├── cpu.rs          # CPU usage monitoring
    │   └── memory.rs       # RAM monitoring
    ├── config/
    │   └── mod.rs          # Settings, skins, persistence (JSON)
    ├── ui/
    │   ├── mod.rs          # UI module exports
    │   ├── settings_dialog.rs  # Options dialog (General/Display/Skin/Network)
    │   └── history_dialog.rs   # Traffic history (daily/monthly)
    └── utils/
        └── mod.rs          # Color interpolation, date helpers
```

## Configuration

Config is stored at:
- **Windows**: `%APPDATA%\TrafficMonitorRs\config.json`
- **Linux**: `~/.config/TrafficMonitorRs/config.json`
- **macOS**: `~/Library/Application Support/TrafficMonitorRs/config.json`

## Usage

- **Drag** the window to reposition
- **Right-click** for the context menu
- **Options** → configure skins, display items, network adapter, refresh rate
- **Traffic History** → view daily/monthly bandwidth usage

## Architecture

| Component | Crate |
|-----------|-------|
| GUI framework | `eframe` + `egui` |
| System metrics | `sysinfo` |
| Serialization | `serde` + `serde_json` |
| Date/time | `chrono` |
| Config directory | `dirs` |
| Logging | `log` + `env_logger` |

## Comparison with Original

| Feature | Original (C++/MFC) | This Rust version |
|---------|-------------------|-------------------|
| Network speed | ✅ | ✅ |
| CPU usage | ✅ | ✅ |
| Memory usage | ✅ | ✅ |
| Skins | ✅ | ✅ (4 built-in + custom editor) |
| History | ✅ | ✅ |
| Taskbar embed | ✅ | 🔜 (planned) |
| Temperature | ✅ (hwmonitor) | 🔜 (planned) |
| GPU usage | ✅ (hwmonitor) | 🔜 (planned) |
| Plugin system | ✅ | 🔜 (planned) |
| DPI aware | ✅ | ✅ (egui auto) |

## License

GPL-3.0 (same as original TrafficMonitor)
