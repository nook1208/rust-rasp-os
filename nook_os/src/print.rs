use core::fmt;
use crate::bsp;

pub fn _print(args: fmt::Arguments) {
    pub use core::fmt::Write;
    bsp::console::console().write_fmt(args).unwrap();
}

/// Print string

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => (crate::print::_print(format_args!($(args)*)))
}

/// Print string with a newline
#[macro_export]
macro_rules! println {
    () => (crate::print::print!("\n"));
    ($($args:tt)*) => ({
        crate::print::_print(format_args_nl!($($args)*));
    })
}
