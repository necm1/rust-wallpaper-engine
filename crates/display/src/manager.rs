use std::collections::HashMap;

use shared::monitor_enum_proc;
use windows::Win32::{
    Foundation::LPARAM,
    Graphics::Gdi::{EnumDisplayMonitors, MONITORINFO},
    UI::WindowsAndMessaging::{GetSystemMetrics, SM_CMONITORS},
};

use crate::DisplayInfo;

pub struct DisplayManager {
    monitors: HashMap<i32, DisplayInfo>,
    monitor_count: i32,
}

impl DisplayManager {
    pub fn scan() -> Result<Self, String> {
        let mut manager = DisplayManager {
            monitors: HashMap::new(),
            monitor_count: 0,
        };

        manager.get_monitors();

        // Should never happen
        if manager.monitors.is_empty() {
            return Err("No monitors found".to_string());
        }

        Ok(DisplayManager {
            monitors: manager.monitors,
            monitor_count: manager.monitor_count,
        })
    }

    pub fn get_primary(&self) -> Result<&DisplayInfo, String> {
        return self
            .monitors
            .values()
            .find(|m| m.left == 0 && m.top == 0)
            .ok_or_else(|| "Primary monitor not found".to_string());
    }

    fn get_monitors(&mut self) {
        unsafe {
            let mut monitors: Vec<MONITORINFO> = Vec::new();

            EnumDisplayMonitors(
                None,
                None,
                Some(monitor_enum_proc),
                LPARAM(&mut monitors as *mut _ as isize),
            );

            for (index, info) in monitors.iter().enumerate() {
                self.add_monitor_info(index, info);
            }
        }
    }

    fn add_monitor_info(&mut self, index: usize, info: &MONITORINFO) {
        let width = info.rcMonitor.right - info.rcMonitor.left;
        let height = info.rcMonitor.bottom - info.rcMonitor.top;
        let left = info.rcMonitor.left;
        let top = info.rcMonitor.top;

        let is_primary = (info.dwFlags & 0x00000001) != 0;

        println!(
            "Monitor {}: {}x{} at ({}, {}), primary: {}",
            index, width, height, left, top, is_primary
        );

        self.monitors.insert(
            index as i32,
            DisplayInfo::new(width, height, index as i32, left, top, is_primary),
        );
    }

    fn get_monitor_count() -> i32 {
        unsafe {
            return GetSystemMetrics(SM_CMONITORS);
        };
    }
}

impl Default for DisplayManager {
    fn default() -> Self {
        DisplayManager {
            monitors: HashMap::new(),
            monitor_count: 0,
        }
    }
}
