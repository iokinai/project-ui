use std::ops::BitOr;

#[repr(C)]
/// Represents window's class style
/// See [this](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles) for more information
pub enum ClassStyle {
    /// Matches the original CS_BYTEALIGNCLIENT
    ByteAlignClient = 0x1000,
    /// Matches the original CS_BYTEALIGNWINDOW
    ByteAlignWindow = 0x2000,
    /// Matches the original CS_CLASSDC
    ClassDC = 0x0040,
    /// Matches the original CS_DBLCLKS
    DoubleClicks = 0x0008,
    /// Matcher the original CS_DROPSHADOW
    DropsShadow = 0x00020000,
    /// Matches the original CS_GLOBALCLASS
    GlobalClass = 0x4000, 
    /// Matches the original CS_HREDRAW
    WidthRedraw = 0x0002,
    /// Matches the original CS_NOCLOSE
    NoClose = 0x0200, 
    /// Matches the original CS_OWNDC
    OwnDeviceContext = 0x0020,
    /// Matches the original CS_PARENTDC
    ParentDeviceContext = 0x0080,
    /// Matches the original CS_SAVEBITS
    SaveBits = 0x0800,
    /// Matches the original CS_VREDRAW
    HeightRedraw = 0x0001,
}

impl BitOr for ClassStyle {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}