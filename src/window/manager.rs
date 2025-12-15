use std::collections::HashMap;

use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, GetSystemMetrics, MSG, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
    TranslateMessage,
};

use crate::{
    display::DisplayManager,
    window::{Desktop, Window},
};

pub struct WindowManager {
    desktop: Desktop,
    displays: DisplayManager,
    windows: HashMap<usize, Window>,
}

impl WindowManager {
    pub fn new() -> Self {
        WindowManager {
            desktop: Desktop::default(),
            displays: DisplayManager::default(),
            windows: HashMap::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        self.desktop = Desktop::find()?;

        self.displays = DisplayManager::scan()?;

        let primary = self.displays.get_primary()?;

        // using only primary for now
        // for testing purposes
        unsafe {
            // TODO: maybe move this one to link?
            let virt_x = GetSystemMetrics(SM_XVIRTUALSCREEN);
            let virt_y = GetSystemMetrics(SM_YVIRTUALSCREEN);

            let progman_child_left = primary.left - virt_x;
            let progman_child_top = primary.top - virt_y;

            let window = Window::create(
                primary.width,
                primary.height,
                progman_child_left,
                progman_child_top,
                self.desktop.get_workerw(),
                self.desktop.get_defview(),
                self.desktop.get_progman(),
            )?;

            self.windows.insert(0, window);

            if let Some(win) = self.windows.get(&0) {
                win.show();
            }
        }

        Ok(())
    }

    pub fn run_loop(&self) -> Result<(), String> {
        unsafe {
            loop {
                let mut msg = MSG::default();
                let result = GetMessageW(&mut msg, None, 0, 0);

                if result.as_bool() {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                } else {
                    break;
                }
            }
            Ok(())
        }
    }
}
