#![feature(rustc_private)]

extern crate term;

mod print_stuff;

pub mod fancy_printer_toolbox {
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
}

fn main() {
    use fancy_printer_toolbox;
    print_stuff::print_title();

    fancy_printer_toolbox::black_and_magenta("A Fancy Sentence; hence the the semi-colon.");
    print_stuff::seperator();
    fancy_printer_toolbox::black_and_white("Something with more flair");
    print_stuff::seperator();
}

#[test]
fn it_works() {
    assert!(true); // is this what I'm supposed to do!?!?
}
