use wchar::wchar_t;
use std::os::raw;

/// Represents Windows' `HANDLE`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type HANDLE = *const raw::c_void;
/// Represents Windows' `HWND`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type HWND = HANDLE;
/// Represents Windows' `UINT`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type UINT = raw::c_uint;
/// Represents Windows' `LONG_PTR`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
#[allow(non_camel_case_types)]
pub type LONG_PTR = raw::c_long;
/// Represents Windows' `UINT_PTR`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
#[allow(non_camel_case_types)]
/// Represents Windows' `UINT_PTR`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type UINT_PTR = UINT;
/// Represents Windows' `LPARAM`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type LPARAM = LONG_PTR;
/// Represents Windows' `WPARAM`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type WPARAM = UINT_PTR;
/// Represents Windows' `HINSTANCE`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type HINSTANCE = HANDLE;
/// Represents Windows' `HMODULE`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type HMODULE = HANDLE;
/// Represents Windows' `HICON`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type HICON = HANDLE;
/// Represents Windows' `HCURSOR`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type HCURSOR = HICON;
/// Represents Windows' `HBRUSH`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type HBRUSH = HANDLE;
/// Represents Windows' `LPCSTR`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type LPCSTR = *const raw::c_char;
/// Represents Windows' `LPCWSTR`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type LPCWSTR = *const wchar_t;
/// Represents Windows' `WNDPROC`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type WNDPROC = fn(HWND, UINT, WPARAM, LPARAM);
/// Represents Windows' `ATOM`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type ATOM = u16;
/// Represents Windows' `BYTE`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type BYTE = u8;
/// Represents Windows' `WORD`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type WORD = u16;
/// Represents Windows' `DWORD`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type DWORD = u32;
/// Represents Windows' `QWORD`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type QWORD = u64;
/// Represents Windows' `HMENU`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type HMENU = HANDLE;
/// Represents Windows' `LPVOID`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types) for more information
pub type LPVOID = *const raw::c_void;