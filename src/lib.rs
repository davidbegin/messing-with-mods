#![feature(rustc_private)]

extern crate term;

pub mod print_stuff;
pub mod fancy_printer_toolbox;
mod ti_83;

#[cfg(test)]
mod tests {
    use super::ti_83;

    #[test]
    fn test_math_in_your_face_returns_255() {
        assert_eq!(ti_83::math_in_your_face(), 255);
    }

    #[test]
    fn test_add_two_numbers_works() {
        assert_eq!(ti_83::add_two_numbers(9, 7), 16);
    }
}
