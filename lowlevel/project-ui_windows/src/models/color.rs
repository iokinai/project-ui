// Represents Windows color type
// See [this](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor#parameters) for more information

// Dark shadow for three-dimensional display elements. Windows 10 or greater: This value is not supported.
pconst!(_3DDKSHADOW, i32, 21);
// Face color for three-dimensional display elements and for dialog box backgrounds.
pconst!(_3DFACE, i32, 15);
// Highlight color for three-dimensional display elements (for edges facing the light source.) Windows 10 or greater: This value is not supported.
pconst!(_3DHIGHLIGHT, i32, 20);
// Highlight color for three-dimensional display elements (for edges facing the light source.) Windows 10 or greater: This value is not supported.
pconst!(_3DHILIGHT, i32, 20);
// Light color for three-dimensional display elements (for edges facing the light source.) Windows 10 or greater: This value is not supported.
pconst!(_3DLIGHT, i32, 22);
// Shadow color for three-dimensional display elements (for edges facing away from the light source). Windows 10 or greater: This value is not supported.
pconst!(_3DSHADOW, i32, 16);
// Active window border. Windows 10 or greater: This value is not supported.
pconst!(ACTIVEBORDER, i32, 10);
// Active window title bar. The associated foreground color is COLOR_CAPTIONTEXT. Specifies the left side color in the color gradient of an active window's title bar if the gradient effect is enabled. Windows 10 or greater: This value is not supported.
pconst!(ACTIVECAPTION, i32, 2);
// Background color of multiple document interface (MDI) applications. Windows 10 or greater: This value is not supported.
pconst!(APPWORKSPACE, i32, 12);
// Desktop. Windows 10 or greater: This value is not supported.
pconst!(BACKGROUND, i32, 1);
// Face color for three-dimensional display elements and for dialog box backgrounds. The associated foreground color is COLOR_BTNTEXT. Windows 10 or greater: This value is not supported.
pconst!(BTNFACE, i32,  15);
// Highlight color for three-dimensional display elements (for edges facing the light source.) Windows 10 or greater: This value is not supported.
pconst!(BTNHIGHLIGHT, i32, 20);
// Highlight color for three-dimensional display elements (for edges facing the light source.) Windows 10 or greater: This value is not supported.
pconst!(BTNHILIGHT, i32, 20);
// Shadow color for three-dimensional display elements (for edges facing away from the light source). Windows 10 or greater: This value is not supported.
pconst!(BTNSHADOW, i32, 16);
// Text on push buttons. The associated background color is COLOR_BTNFACE.
pconst!(BTNTEXT, i32, 18);
// Text in caption, size box, and scroll bar arrow box. The associated background color is COLOR_ACTIVECAPTION. Windows 10 or greater: This value is not supported.
pconst!(CAPTIONTEXT, i32, 9);
// Desktop. Windows 10 or greater: This value is not supported.
pconst!(DESKTOP, i32, 1);
// Right side color in the color gradient of an active window's title bar. COLOR_ACTIVECAPTION specifies the left side color. Use SPI_GETGRADIENTCAPTIONS with the SystemParametersInfo function to determine whether the gradient effect is enabled. Windows 10 or greater: This value is not supported.
pconst!(GRADIENTACTIVECAPTION, i32, 27);
// Right side color in the color gradient of an inactive window's title bar. COLOR_INACTIVECAPTION specifies the left side color Windows 10 or greater: This value is not supported.
pconst!(GRADIENTINACTIVECAPTION, i32, 28);
// Grayed (disabled) text. This color is set to 0 if the current display driver does not support a solid gray color
pconst!(GRAYTEXT, i32, 17);
// Item(s) selected in a control. The associated foreground color is COLOR_HIGHLIGHTTEXT
pconst!(HIGHLIGHT, i32, 13);
// Text of item(s) selected in a control. The associated background color is COLOR_HIGHLIGHT.
pconst!(HIGHLIGHTTEXT, i32, 14);
// Color for a hyperlink or hot-tracked item. The associated background color is COLOR_WINDOW
pconst!(HOTLIGHT, i32, 26);
// Inactive window border. Windows 10 or greater: This value is not supported.
pconst!(INACTIVEBORDER, i32, 11);
// Inactive window caption. The associated foreground color is COLOR_INACTIVECAPTIONTEXT. Specifies the left side color in the color gradient of an inactive window's title bar if the gradient effect is enabled. Windows 10 or greater: This value is not supported.
pconst!(INACTIVECAPTION, i32, 3);
// Color of text in an inactive caption. The associated background color is COLOR_INACTIVECAPTION. Windows 10 or greater: This value is not supported.
pconst!(INACTIVECAPTIONTEXT, i32, 19);
// Background color for tooltip controls. The associated foreground color is COLOR_INFOTEXT. Windows 10 or greater: This value is not supported
pconst!(INFOBK, i32, 24);
// Text color for tooltip controls. The associated background color is COLOR_INFOBK. Windows 10 or greater: This value is not supported.
pconst!(INFOTEXT, i32, 23);
// Menu background. The associated foreground color is COLOR_MENUTEXT. Windows 10 or greater: This value is not supported
pconst!(MENU, i32, 4);
// The color used to highlight menu items when the menu appears as a flat menu (see SystemParametersInfo). The highlighted menu item is outlined with COLOR_HIGHLIGHT. Windows 2000, Windows 10 or greater:  This value is not supported.
pconst!(MENUHILIGHT, i32, 29);
// The background color for the menu bar when menus appear as flat menus (see SystemParametersInfo). However, COLOR_MENU continues to specify the background color of the menu popup. Windows 2000, Windows 10 or greater:  This value is not supported.
pconst!(MENUBAR, i32, 30);
// Text in menus. The associated background color is COLOR_MENU. Windows 10 or greater: This value is not supported.
pconst!(MENUTEXT, i32, 7);
// Scroll bar gray area. Windows 10 or greater: This value is not supported.
pconst!(SCROLLBAR, i32, 0);
// Window background. The associated foreground colors are COLOR_WINDOWTEXT and COLOR_HOTLITE.
pconst!(WINDOW, i32, 5);
// Window frame. Windows 10 or greater: This value is not supported.
pconst!(WINDOWFRAME, i32, 6);
// Text in windows. The associated background color is COLOR_WINDOW
pconst!(WINDOWTEXT, i32, 8);