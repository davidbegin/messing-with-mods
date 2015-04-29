// some mod makes something like a module in ruby I'm guessing
// so lets run with that

// I also am going to steal the code from here
// https://doc.rust-lang.org/term/term/index.html
//
// and try and understand it


// Well Well Well
// Good ole unstable Rust!
//
// [ERROR FOR POSTERITY]
// help: add #![feature(rustc_private)] to the crate attributes to enable
// error: use of unstable library feature 'rustc_private': use the crates.io `term`
//
// adding this is admitting defeat
//
// the battle is lost
//
// ...but not the war
#![feature(rustc_private)]

extern crate term;
use std::io::prelude::*;

fn main() {
    print_title();

    let mut somehow_this_is_stdout_and_it_has_to_mutable_eh = term::stdout().unwrap();

    // I'm guessing fg is foreground
    // so I almost just want to try out bg first
    //
    // ...but alas I will wait
    somehow_this_is_stdout_and_it_has_to_mutable_eh.fg(term::color::GREEN).unwrap();
    (write!(somehow_this_is_stdout_and_it_has_to_mutable_eh, "hello, ")).unwrap();

    somehow_this_is_stdout_and_it_has_to_mutable_eh.fg(term::color::RED).unwrap();
    (write!(somehow_this_is_stdout_and_it_has_to_mutable_eh, "world!")).unwrap();

    somehow_this_is_stdout_and_it_has_to_mutable_eh.reset().unwrap();

    seperator();
}

fn print_title() {
    println!("\n\nMessing with Modules");
    println!("====================\n\n");
}

fn seperator() {
    println!("\n")
}

#[test]
fn it_works() {
    assert!(true); // is this what I'm supposed to do!?!?
}
