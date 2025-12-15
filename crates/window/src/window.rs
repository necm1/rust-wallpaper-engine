use shared::window_proc;
use windows::{
    Win32::{
        Foundation::{COLORREF, HWND},
        Graphics::Gdi::CreateSolidBrush,
        System::LibraryLoader::GetModuleHandleA,
        UI::WindowsAndMessaging::{
            CS_HREDRAW, CS_VREDRAW, CreateWindowExW, GWL_EXSTYLE, GWL_STYLE, GetWindowLongA,
            IDC_ARROW, IDI_APPLICATION, LWA_ALPHA, LoadCursorW, LoadIconW, RegisterClassW, SW_SHOW,
            SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetLayeredWindowAttributes,
            SetParent, SetWindowLongA, SetWindowPos, ShowWindow, WNDCLASSW, WS_CAPTION, WS_CHILD,
            WS_EX_CLIENTEDGE, WS_EX_DLGMODALFRAME, WS_EX_LAYERED, WS_EX_STATICEDGE,
            WS_EX_WINDOWEDGE, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_POPUP, WS_SYSMENU, WS_THICKFRAME,
            WS_VISIBLE,
        },
    },
    core::w,
};
pub struct Window {
    pub hwnd: HWND,
}

impl Window {
    pub fn create(
        width: i32,
        height: i32,
        left: i32,
        top: i32,
        parent: HWND,
        defview: HWND,
        progman: HWND,
    ) -> Result<Self, String> {
        let window = Self::create_window(width, height)?;

        window.remove_window_decorations()?;
        window.attach_to_desktop(parent, progman, defview, width, height, left, top)?;

        Ok(window)
    }

    fn create_window(width: i32, height: i32) -> Result<Self, String> {
        unsafe {
            let class_name = w!("Rust Wallpaper Engine");
            let h_instance = GetModuleHandleA(None).ok().unwrap_or_default();

            let brush = CreateSolidBrush(COLORREF(0x000000FF));

            let wnd_class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: h_instance.into(),
                hIcon: LoadIconW(None, IDI_APPLICATION).ok().unwrap_or_default(),
                hCursor: LoadCursorW(None, IDC_ARROW).ok().unwrap_or_default(),
                hbrBackground: brush,
                lpszMenuName: w!(""),
                lpszClassName: class_name,
            };

            RegisterClassW(&wnd_class);

            let hwnd = CreateWindowExW(
                WS_EX_LAYERED,
                class_name,
                w!("Rust Wallpaper"),
                WS_POPUP | WS_VISIBLE,
                0,
                0,
                width,
                height,
                HWND(std::ptr::null_mut()),
                None,
                h_instance,
                None,
            )
            .map_err(|e| format!("CreateWindowExW failed: {:?}", e))?;

            Ok(Window { hwnd })
        }
    }

    fn remove_window_decorations(&self) -> Result<(), String> {
        unsafe {
            let mut style = GetWindowLongA(self.hwnd, GWL_STYLE);

            let remove_style =
                WS_CAPTION.0 | WS_THICKFRAME.0 | WS_SYSMENU.0 | WS_MAXIMIZEBOX.0 | WS_MINIMIZEBOX.0;
            style &= !(remove_style as i32);

            let mut exstyle = GetWindowLongA(self.hwnd, GWL_EXSTYLE);

            let remove_exstyle = WS_EX_DLGMODALFRAME.0
                | WS_EX_WINDOWEDGE.0
                | WS_EX_CLIENTEDGE.0
                | WS_EX_STATICEDGE.0;
            exstyle &= !(remove_exstyle as i32);

            style &= !(WS_POPUP.0 as i32);

            style |= WS_CHILD.0 as i32;

            SetWindowLongA(self.hwnd, GWL_STYLE, style);
            SetWindowLongA(self.hwnd, GWL_EXSTYLE, exstyle);

            Ok(())
        }
    }

    fn attach_to_desktop(
        &self,
        parent: HWND,
        progman: HWND,
        defview: HWND,
        width: i32,
        height: i32,
        left: i32,
        top: i32,
    ) -> Result<(), String> {
        unsafe {
            // @source https://github.com/rocksdanister/lively/issues/2074#issuecomment-3017842549

            // set parent window to Progman
            SetParent(self.hwnd, progman).map_err(|e| format!("SetParent failed: {:?}", e))?;

            // insert below icon window
            SetWindowPos(
                self.hwnd,
                defview,
                left,
                top,
                width,
                height,
                SWP_NOACTIVATE | SWP_SHOWWINDOW,
            )
            .map_err(|e| format!("SetWindowPos failed: {:?}", e))?;

            // push WorkerW behind our window
            SetWindowPos(
                parent,
                self.hwnd,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            )
            .map_err(|e| format!("SetWindowPos failed: {:?}", e))?;

            // let the magic happen
            // we have to call SetLayeredWindowAttributes or UpdateLayeredWindow here
            // to make it visible...
            SetLayeredWindowAttributes(self.hwnd, COLORREF(0), 255, LWA_ALPHA)
                .map_err(|e| format!("SetLayeredWindowAttributes failed: {:?}", e))?;

            Ok(())
        }
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
    }
}
