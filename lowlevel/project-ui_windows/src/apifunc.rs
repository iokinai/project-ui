use crate::models::{WNDCLASSEXA, types::{ATOM, LPCSTR, HMODULE, HINSTANCE, HICON, HCURSOR, HWND, UINT, DWORD, LPVOID, HMENU}};

#[link(name = "kernel32", kind = "dylib")]
#[link(name = "user32", kind = "dylib")]
extern "stdcall" {
    pub fn RegisterClassExA(cl: WNDCLASSEXA) -> ATOM;
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    pub fn LoadIconA(hInstance: HINSTANCE, lpIconName: LPCSTR) -> HICON;
    pub fn LoadCursorA(hInstance: HINSTANCE, lpCursorName: LPCSTR) -> HCURSOR;
    pub fn MessageBoxA(hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> i32;
    pub fn CreateWindowExA(
        dwExStyle: DWORD,
        lpClassName: LPCSTR,
        lpWindowName: LPCSTR,
        dwStyle: DWORD,
        X: i32,
        Y: i32,
        nWindth: i32,
        nHeight: i32,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> bool;
    pub fn UpdateWindow(hWnd: HWND) -> bool;
}