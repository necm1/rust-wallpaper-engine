use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::{
    BeginPaint, CreateSolidBrush, DeleteObject, EndPaint, FillRect, GetMonitorInfoW, HDC, HMONITOR,
    MONITORINFO, PAINTSTRUCT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, FindWindowExW, GetClientRect, PostQuitMessage, WM_DESTROY, WM_ERASEBKGND,
    WM_PAINT,
};
use windows::core::w;

pub unsafe extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _lprect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    let monitors: &mut Vec<MONITORINFO> = &mut *(lparam.0 as *mut Vec<MONITORINFO>);

    let mut info = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };

    if GetMonitorInfoW(hmonitor, &mut info).as_bool() {
        monitors.push(info);
    }
    true.into()
}

pub unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let parent_ptr = lparam.0 as *mut HWND;

    let defview = FindWindowExW(
        hwnd,
        HWND(std::ptr::null_mut()),
        w!("SHELLDLL_DefView"),
        None,
    );

    if defview.is_ok() && !defview.unwrap().0.is_null() {
        *parent_ptr = hwnd;
        println!("Found SHELLDLL_DefView under: {:?}", hwnd);
        return FALSE;
    }
    TRUE
}

pub unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_ERASEBKGND => LRESULT(1),
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);

            let mut rect = RECT::default();
            GetClientRect(hwnd, &mut rect);

            println!(
                "Painting rect: left={}, top={}, right={}, bottom={}",
                rect.left, rect.top, rect.right, rect.bottom
            );

            let brush = CreateSolidBrush(COLORREF(0x000000FF)); // Rot: 0x00BBGGRR
            FillRect(hdc, &rect, brush);
            DeleteObject(brush);

            EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
