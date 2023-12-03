mod wnd_class;
mod class_style;
mod wnd_class_ex;

pub mod types;
pub mod icon_display_indentifier;
pub mod identifier_of_control;

#[macro_use]
pub mod macros;

pub mod color;
pub mod window_style;

/// 
pub use wnd_class::tagWNDCLASSA;
///
pub use class_style::ClassStyle;
///
pub use wnd_class_ex::tagWNDCLASSEXA;
///
pub use wnd_class_ex::WNDCLASSEXA;
///
pub use icon_display_indentifier as IDI;
///
pub use identifier_of_control as IDC;
///
pub use color as Color;
///
pub use window_style as WS;