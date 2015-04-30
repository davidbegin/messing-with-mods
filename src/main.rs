extern crate messin_with_mods;

mod print_stuff;
mod fancy_printer_toolbox;

fn main() {
    print_stuff::print_title();
    fancy_printer_toolbox::black_and_magenta("A Fancy Sentence; hence the the semi-colon.");
    print_stuff::seperator();
    fancy_printer_toolbox::black_and_white("Something with more flair");
    print_stuff::seperator();
}
