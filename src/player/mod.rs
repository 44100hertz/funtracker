pub mod channel;
pub mod song;
pub mod base32;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::parser;
    use super::base32;
    #[test]
    fn middle_c() {
        let note = parser::parse_note("C-4");
        assert_eq!(note, 48)
    }

    #[test]
    fn a_sharp_0() {
        let note = parser::parse_note("A#0");
        assert_eq!(note, 10)
    }

    #[test]
    fn invalid_note() {
        let note = parser::parse_note("a lemon");
        assert_eq!(note, -1)
    }

    #[test]
    fn base32_zero() {
        let zero = base32::char_to_base32('o');
        assert_eq!(zero, '0')
    }
}
