// Standard Windows Icons
// See [this](https://learn.microsoft.com/en-us/windows/win32/menurc/about-icons#icon-types) for more information

use crate::{MAKEINTRESOURCE, pconst};

// Default application icon
pconst!(APPLICATION, *const i8, MAKEINTRESOURCE!(32512));
// Error icon
pconst!(ERROR, *const i8, MAKEINTRESOURCE!(32513));
// Question mark icon
pconst!(QUESTION, *const i8, MAKEINTRESOURCE!(32514));
// Warning icon
pconst!(WARNING, *const i8, MAKEINTRESOURCE!(32515));
// Information icon
pconst!(INFORMATION, *const i8, MAKEINTRESOURCE!(32516));
// Windows logo icon
pconst!(WINLOGO, *const i8, MAKEINTRESOURCE!(32517));
// Security shield icon
pconst!(SHIELD, *const i8, MAKEINTRESOURCE!(32518));