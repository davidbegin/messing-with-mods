#![feature(rustc_private)]
#![feature(core)]

extern crate term;
use std::io::prelude::*;

pub mod fancy_printer_toolbox {
    extern crate term;
    use std::io::prelude::*;

    pub fn fancy_print(string: &'static str) {
        let mut t = term::stdout().unwrap();
        t.fg(term::color::BLACK).unwrap();
        t.bg(term::color::BRIGHT_MAGENTA).unwrap();
        (write!(t, "{}", string)).unwrap();
        t.reset().unwrap();
    }
}

fn main() {
    use fancy_printer_toolbox;
    print_title();

    fancy_printer_toolbox::fancy_print("A Fancy Sentence; hence the the semi-colon.");
    seperator();
}

fn print_title() {
    println!("\n\nMessing with Modules");
    println!("====================\n\n");
}

fn seperator() {
    println!("\n");
}

#[test]
fn it_works() {
    assert!(true); // is this what I'm supposed to do!?!?
}
