#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]
#![deny(warnings)]

use pyo3::prelude::*;

#[macro_export]
macro_rules! print_header {
    ($($arg:tt)*) => {
        println!("\x1b[34m{}\x1b[0m", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {
        eprintln!("\x1b[32msuccess: {}\x1b[0m", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => {
        eprintln!("\x1b[33mwarning: {}\x1b[0m", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        eprintln!("\x1b[31merror: {}\x1b[0m", format!($($arg)*))
    };
}

pub mod prelude;

mod gnuplot;
mod system;
mod text;

#[pymodule(name = "utilities")]
pub mod utilities {}
