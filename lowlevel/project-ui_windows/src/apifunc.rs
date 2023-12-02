use crate::models::{WNDCLASSEXA, types::{ATOM, LPCSTR, HMODULE, HINSTANCE, HICON}};

#[link(name = "kernel32", kind = "dylib")]
#[link(name = "user32", kind = "dylib")]
extern "stdcall" {
    pub fn RegisterClassExA(cl: WNDCLASSEXA) -> ATOM;
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    pub fn LoadIconA(hInstance: HINSTANCE, lpIconName: LPCSTR) -> HICON;
}