pub mod channel;
pub mod song;
pub mod base32;
pub mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn middle_c() {
        let note = super::parser::parse_note("C-4");
        assert_eq!(note, 48)
    }

    #[test]
    fn a_sharp_0() {
        let note = super::parser::parse_note("A#0");
        assert_eq!(note, 10)
    }
}
