// Standard Windows Cursors
// See [this](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors) for more information

use crate::{MAKEINTRESOURCE, pconst};

// Normal select
pconst!(ARROW, *const i8, MAKEINTRESOURCE!(32512));
// Text select
pconst!(IBEAM, *const i8, MAKEINTRESOURCE!(32513));
// Busy
pconst!(WAIT, *const i8, MAKEINTRESOURCE!(32514));
// Precision select
pconst!(CROSS, *const i8, MAKEINTRESOURCE!(32515));
// Alternative select
pconst!(UPARROW, *const i8, MAKEINTRESOURCE!(32516));
// Handwriting
pconst!(HANDWRITING, *const i8, MAKEINTRESOURCE!(32631));
// Diagonal resize 1
pconst!(SIZENWSE, *const i8, MAKEINTRESOURCE!(32642));
// Diagonal resize 2
pconst!(SIZENESW, *const i8, MAKEINTRESOURCE!(32643));
// Horizontal resize
pconst!(SIZEWE, *const i8, MAKEINTRESOURCE!(32644));
// Vertical resize
pconst!(SIZENS, *const i8, MAKEINTRESOURCE!(32645));
// Move
pconst!(SIZEALL, *const i8, MAKEINTRESOURCE!(32646));
// Unavailable 
pconst!(NO, *const i8, MAKEINTRESOURCE!(32648));
// Link select
pconst!(HAND, *const i8, MAKEINTRESOURCE!(32649));
// Working in background
pconst!(APPSTARTING, *const i8, MAKEINTRESOURCE!(32650));
// Help select
pconst!(HELP, *const i8, MAKEINTRESOURCE!(32651));
// Location select
pconst!(PIN, *const i8, MAKEINTRESOURCE!(32671));
// Person select
pconst!(PERSON, *const i8, MAKEINTRESOURCE!(32672));
