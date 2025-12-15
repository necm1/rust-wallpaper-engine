use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{EnumWindows, FindWindowExW, FindWindowW, SendMessageA},
    },
    core::w,
};

use crate::window::enum_windows_proc;

pub struct Desktop {
    progman: HWND,
    workerw: HWND,
    defview: HWND,
}

impl Desktop {
    pub fn find() -> Result<Self, String> {
        unsafe {
            let progman = FindWindowW(w!("Progman"), None)
                .map_err(|e| format!("FindWindowW Progman failed: {:?}", e))?;

            println!("Found Progman: {:?}", progman);

            // Spawn WorkerW windows - weebp as ref
            SendMessageA(progman, 0x052C, WPARAM(0xD), LPARAM(0));
            SendMessageA(progman, 0x052C, WPARAM(0xD), LPARAM(1));

            let mut defview_parent = HWND(std::ptr::null_mut());

            // ignore EnumWindows possible "error"
            EnumWindows(
                Some(enum_windows_proc),
                LPARAM(&mut defview_parent as *mut _ as isize),
            );

            if defview_parent.0.is_null() {
                return Err("SHELLDLL_DefView parent not found".to_string());
            }

            let defview = FindWindowExW(
                defview_parent,
                HWND(std::ptr::null_mut()),
                w!("SHELLDLL_DefView"),
                None,
            )
            .unwrap();

            println!(
                "DefView parent: {:?}, DefView: {:?}",
                defview_parent, defview
            );

            Ok(Desktop {
                workerw: defview_parent,
                defview,
                progman,
            })
        }
    }

    pub fn get_workerw(&self) -> HWND {
        self.workerw
    }

    pub fn get_progman(&self) -> HWND {
        self.progman
    }

    pub fn get_defview(&self) -> HWND {
        self.defview
    }
}

impl Default for Desktop {
    fn default() -> Self {
        Desktop {
            progman: HWND(std::ptr::null_mut()),
            workerw: HWND(std::ptr::null_mut()),
            defview: HWND(std::ptr::null_mut()),
        }
    }
}
