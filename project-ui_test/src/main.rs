use std::ffi::CString;
use std::mem::size_of;
use std::ptr::null;

use project_ui_windows::models::{WNDCLASSEXA, ClassStyle, IDI, IDC, Color, WS};
use project_ui_windows::models::types::{HWND, UINT, WPARAM, LPARAM, HBRUSH};
use project_ui_windows::apifunc::{RegisterClassExA, GetModuleHandleA, LoadIconA, LoadCursorA, MessageBoxA, CreateWindowExA, ShowWindow, UpdateWindow};

#[allow(non_snake_case)]
fn main() {
    let error_string: CString = CString::new("Error!").unwrap();
    let hInstance = unsafe { GetModuleHandleA(null()) };

    let cstring = CString::new("DesktopApp").unwrap();

    let class = WNDCLASSEXA {
        cbSize: size_of::<WNDCLASSEXA>() as u32,
        style: (ClassStyle::WidthRedraw | ClassStyle::HeightRedraw),
        lpfnWndProc: wnd_proc,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance,
        hIcon: unsafe { LoadIconA(hInstance, IDI::APPLICATION) },
        hCursor: unsafe { LoadCursorA(null(), IDC::ARROW) },
        hbrBackground: (Color::WINDOW + 1) as HBRUSH,
        lpszMenuName: null(),
        lpszClassName: cstring.as_ptr(),
        hIconSm: unsafe { LoadIconA(hInstance, IDI::APPLICATION) },
    };

    unsafe {
        let mbtext = CString::new("Error on registring class").unwrap();
        if RegisterClassExA(class) == 0 {
            MessageBoxA(null(), mbtext.as_ptr(), error_string.as_ptr(), 0x00000001);
            return;
        }

        let wndname = CString::new("Test app").unwrap();

        let main_window = CreateWindowExA(WS::OVERLAPPED | WS::MINIMIZEBOX | WS::MAXIMIZEBOX, cstring.as_ptr(), wndname.as_ptr(), WS::OVERLAPPED | WS::MINIMIZEBOX | WS::MAXIMIZEBOX, 0, 0, 500, 100, null(), null(), hInstance, null());

        let wnderrortext = CString::new("Error on creating window").unwrap();
        if main_window == null() {
            println!("error");
            MessageBoxA(null(), wnderrortext.as_ptr(), error_string.as_ptr(), 0x00000001);
            return;
        }

        if !ShowWindow(main_window, 5) {
            println!("error");
        }

        UpdateWindow(main_window);

        drop(wndname);
        drop(wnderrortext);
    }

    drop(cstring);
}

#[allow(unused_variables)]
fn wnd_proc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) {
    println!("created");
}