use crate::models::types::{HINSTANCE, HICON, HBRUSH, LPCSTR};

use super::types::WNDPROC;

/// Represents `WNDCLASS` structure
/// See [this](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassa) for more information
#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct tagWNDCLASSA {
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCSTR,
    pub lpszClassName: LPCSTR,
}



