pub mod channel;
pub mod song;
pub mod track;
pub mod note;
pub mod base32;
pub mod parser;
pub mod parser_driver;

#[cfg(test)]
mod parser_tests {
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
        let field = parser::parse_field("C-4 .").unwrap();
        assert_eq!(field.note, Some(48));
    }

    #[test]
    fn parse_command_field() {
        let field = parser::parse_field(".-. A1234").unwrap();
        assert_eq!(field.command, Some('A'));
    }

    #[test]
    fn parse_command_value_field() {
        let field = parser::parse_field(".-. A1234").unwrap();
        assert_eq!(field.value, 1234.0);
    }

    #[test]
    fn parse_kilo() {
        let kilo = parser::parse_num("1k");
        assert_eq!(kilo, 1000.0);
    }

    #[test]
    fn parse_kilo_decimal() {
        let kilo = parser::parse_num("1.1k");
        assert_eq!(kilo, 1100.0)
    }
}

mod note_tests {
    #[test]
    fn note_period() {
        let freq = super::note::get_freq(60.0);
        assert_eq!(freq, 1.0)
    }
}
