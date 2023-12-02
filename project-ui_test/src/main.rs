use std::mem::size_of;
use std::ptr::null;

use project_ui_windows::models::{WNDCLASSEXA, ClassStyle};
use project_ui_windows::models::types::{HINSTANCE, HWND, UINT, WPARAM, LPARAM};
use project_ui_windows::apifunc::{RegisterClassExA, GetModuleHandleA, LoadIconA};

fn main() {
    let hInstance = unsafe { GetModuleHandleA(null()) }

    let class = WNDCLASSEXA {
        cbSize: size_of::<WNDCLASSEXA>() as u32,
        style: (ClassStyle::WidthRedraw | ClassStyle::HeightRedraw),
        lpfnWndProc: wnd_proc,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance,
        hIcon: LoadIconA(hInstance, lpIconName),
        hCursor: todo!(),
        hbrBackground: todo!(),
        lpszMenuName: todo!(),
        lpszClassName: todo!(),
        hIconSm: todo!(),
    };
}

fn wnd_proc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) {

}