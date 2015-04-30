extern crate term;
use std::io::prelude::*;

pub fn black_and_magenta(string: &'static str) {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::BLACK).unwrap();
    t.bg(term::color::BRIGHT_MAGENTA).unwrap();
    (write!(t, "{}", string)).unwrap();
    t.reset().unwrap();
}

pub fn black_and_white(string: &'static str) {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::BLACK).unwrap();
    t.bg(term::color::WHITE).unwrap();
    (write!(t, "{}", string)).unwrap();
    t.reset().unwrap();
}
