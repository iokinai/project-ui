use crate::models::types::{HINSTANCE, HICON, HBRUSH, UINT, LPCSTR};

use super::types::{WNDPROC, HCURSOR};

/// Represents `WNDCLASSEX` structure
/// See [this](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassa) for more information
#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct tagWNDCLASSEXA {
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE, 
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCSTR,
    pub lpszClassName: LPCSTR,
    pub hIconSm: HICON
}

pub type WNDCLASSEXA = tagWNDCLASSEXA;