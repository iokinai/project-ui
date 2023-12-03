/// Converts `$v` to `$v as *const i8`
/// See [this](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcea) for more information
#[macro_export]
macro_rules! MAKEINTRESOURCE {
    ($v: literal) => {
        $v as *const i8
    };
}

/// Creates pub constant
#[macro_export]
macro_rules! pconst {
    ($name: ident, $type: ty, $v: expr) => {
        pub const $name: $type = $v;
    };
}


/// Generates a list of variables
/// The following code:
/// ```
/// vars! {
///     var1 = 1,
///     var2 = 2,
/// }
/// ```
/// will expand to 
/// ```
/// let var1 = 1;
/// let var2 = 2;
/// ```
/// 
#[macro_export]
#[deprecated]
macro_rules! vars {
    ($name: tt: $type: ty = $val: expr) => {
        {let $name = $val};
    };

    ($($name: tt: $type: ty = $val: expr),+) => {
        $(vars!($name: $type = $val);)+
    };
}