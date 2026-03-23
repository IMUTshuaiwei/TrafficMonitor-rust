// src/tray.rs  –  系统托盘（通知区图标）+ 中文菜单
// 不在任务栏显示窗口按钮（由 main.rs 的 with_skip_taskbar(true) 控制）

#[derive(Debug, Clone, PartialEq)]
pub enum TrayEvent {
    ShowOptions,
    Exit,
}

// ─── Windows 实现 ─────────────────────────────────────────────────────────────
#[cfg(target_os = "windows")]
mod imp {
    use super::TrayEvent;
    use tray_icon::{
        TrayIcon, TrayIconBuilder,
        menu::{Menu, MenuItem, PredefinedMenuItem, MenuEvent},
    };

    pub struct AppTray {
        _icon: TrayIcon,
        id_options: String,
        id_exit:    String,
    }

    impl AppTray {
        pub fn new() -> Option<Self> {
            let icon = make_icon()?;
            let menu = Menu::new();

            let item_options = MenuItem::new("⚙ 选项设置...", true, None);
            let item_exit    = MenuItem::new("✖ 退出",        true, None);

            let _ = menu.append(&item_options);
            let _ = menu.append(&PredefinedMenuItem::separator());
            let _ = menu.append(&item_exit);

            let id_options = item_options.id().0.clone();
            let id_exit    = item_exit.id().0.clone();

            let tray = TrayIconBuilder::new()
                .with_menu(Box::new(menu))
                .with_tooltip("流量监控")
                .with_icon(icon)
                .build()
                .ok()?;

            Some(Self { _icon: tray, id_options, id_exit })
        }

        pub fn poll(&self) -> Option<TrayEvent> {
            if let Ok(ev) = MenuEvent::receiver().try_recv() {
                let id = ev.id.0.clone();
                if id == self.id_options { return Some(TrayEvent::ShowOptions); }
                if id == self.id_exit    { return Some(TrayEvent::Exit); }
            }
            None
        }
    }

    /// 生成 16×16 程序图标（无需外部文件）
    fn make_icon() -> Option<tray_icon::Icon> {
        const S: u32 = 16;
        let mut rgba = Vec::with_capacity((S * S * 4) as usize);
        for y in 0..S {
            for x in 0..S {
                let border  = x == 0 || x == S-1 || y == 0 || y == S-1;
                let bar_up  = x >= 2 && x <= 6  && y >= S.saturating_sub(2 + (x - 2) * 2);
                let bar_dn  = x >= 8 && x <= 13 && y >= 2 && y < 2 + (x - 8);
                let (r, g, b, a) = if border     { (60,  60,  60,  255) }
                                   else if bar_up { (80,  210, 80,  255) }
                                   else if bar_dn { (80,  150, 255, 255) }
                                   else           { (18,  18,  24,  220) };
                rgba.extend_from_slice(&[r, g, b, a]);
            }
        }
        tray_icon::Icon::from_rgba(rgba, S, S).ok()
    }
}

// ─── 非 Windows 空实现 ────────────────────────────────────────────────────────
#[cfg(not(target_os = "windows"))]
mod imp {
    use super::TrayEvent;
    pub struct AppTray;
    impl AppTray {
        pub fn new() -> Option<Self> { Some(Self) }
        pub fn poll(&self) -> Option<TrayEvent> { None }
    }
}

pub use imp::AppTray;
