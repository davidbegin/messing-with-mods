#![feature(rustc_private)]

extern crate term;

mod print_stuff;
mod fancy_printer_toolbox;
mod ti_83;

fn main() {
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
