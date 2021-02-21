use std::os::raw::c_void;

use winapi::shared::minwindef::WPARAM;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::PostMessageW;

use super::{_sapp_win32_hwnd, WM_CUSTOM_EVENT};

pub unsafe fn post_custom_event(event_data: *mut c_void) {
    // Both WPARAM and LPARAM are apprarently pointer sized.
    // WPARAM is UINT_PTR and LPARAM is LONG_PTR. So they're the same except
    // LPARAM is signed.
    if PostMessageW(_sapp_win32_hwnd, WM_CUSTOM_EVENT, event_data as WPARAM, 0) == 0 {
        let dw = GetLastError();
        panic!("Failed to PostMessage for custom event data. GetLastError: {:#010x}", dw);
    }
}
