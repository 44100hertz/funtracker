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
        assert_eq!(note, Some(48))
    }

    #[test]
    fn a_sharp_0() {
        let note = parser::parse_note("A#0");
        assert_eq!(note, Some(10))
    }

    #[test]
    fn invalid_note() {
        let note = parser::parse_note("rip");
        assert_eq!(note, None)
    }

    #[test]
    fn base32_zero() {
        let zero = base32::char_to_base32('o');
        assert_eq!(zero, Some('0'))
    }

    #[test]
    fn parse_note_field() {
        let field = parser::parse_field("C-4 .....");
        assert_eq!(field.note, Some(48));
    }

    #[test]
    fn parse_command_field() {
        let field = parser::parse_field(".-. A1234");
        assert_eq!(field.command, Some('A'));
    }

    #[test]
    fn parse_command_value_field() {
        let field = parser::parse_field(".-. A1234");
        assert_eq!(field.value, Some(1234f32));
    }
}