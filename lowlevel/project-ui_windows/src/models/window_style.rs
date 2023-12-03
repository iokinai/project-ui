//  Represents Window style
// See [this](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles) for more information

pconst!(BORDER, u32, 0x00800000u32);
pconst!(CAPTION, u32, 0x00C00000);
pconst!(CHILD, u32, 0x40000000);
pconst!(CHILDWINDOW, u32, 0x40000000);
pconst!(CLIPCHILDREN, u32, 0x02000000);
pconst!(CLIPSIBLINGS, u32, 0x04000000);
pconst!(DISABLED, u32, 0x08000000);
pconst!(DLGBRAME, u32, 0x00400000);
pconst!(GROUP, u32, 0x00020000);
pconst!(HSCROLL, u32, 0x00100000);
pconst!(ICONIC, u32, 0x20000000);
pconst!(MAXIMIZE, u32, 0x01000000);
pconst!(MAXIMIZEBOX, u32, 0x00010000);
pconst!(MINIMIZE, u32, 0x20000000);
pconst!(MINIMIZEBOX, u32, 0x00020000);
pconst!(OVERLAPPED, u32, 0x00000000);
pconst!(POPUP, u32, 0x80000000);
pconst!(SIZEBOX, u32, 0x00040000);
pconst!(SYSMENU, u32, 0x00080000);
pconst!(TABSTOP, u32, 0x00010000);
pconst!(THICKFRAME, u32, 0x00040000);
pconst!(TILED, u32, 0x00000000);
pconst!(VISIBLE, u32, 0x10000000);
pconst!(VSCROLL, u32, 0x00200000);
