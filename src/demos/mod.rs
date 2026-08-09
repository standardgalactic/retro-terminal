mod ansi;
mod boot;
mod colors;
mod cursor;
mod screen;

pub use ansi::ansi_demo;
pub use boot::boot_demo;
pub use colors::color_demo;
pub use cursor::cursor_demo;
pub use screen::{character_demo, matrix_demo, run_demo, scroll_demo, terminal_test, wrap_demo};
