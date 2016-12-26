pub mod channel;
pub mod song;
pub mod track;
pub mod note;
pub mod base32;
pub mod parse;
pub mod files;
pub mod command;

#[cfg(test)]
mod parse_tests {
    use super::parse;
    use super::base32;
    #[test]
    fn middle_c() {
        let note = parse::parse_note("C-4");
        assert_eq!(note, Some(48))
    }

    #[test]
    fn a_sharp_0() {
        let note = parse::parse_note("A#0");
        assert_eq!(note, Some(10))
    }

    #[test]
    fn invalid_note() {
        let note = parse::parse_note("rip");
        assert_eq!(note, None)
    }

    #[test]
    fn base32_zero() {
        let zero = base32::char_to_base32('o');
        assert_eq!(zero, Some('0'))
    }

    #[test]
    fn parse_note_field() {
        let field = parse::parse_field("C-4");
        assert_eq!(field.note, Some(48));
    }

    #[test]
    fn parse_command_field() {
        let field = parse::parse_field(".-. A1234");
        assert_eq!(field.command.unwrap(), "A1234");
    }

    #[test]
    fn parse_dirty_field() {
        let field = parse::parse_field("Cöç ^&@A321™");
        assert_eq!(field.command.unwrap(), "A321");
    }

    #[test]
    fn parse_kilo() {
        let kilo = parse::parse_num("1K");
        assert_eq!(kilo, Some(1000.0));
    }

    #[test]
    fn parse_kilo_decimal() {
        let kilo = parse::parse_num("1.1K");
        assert_eq!(kilo, Some(1100.0))
    }

    #[test]
    fn parse_bad() {
        let wrong = parse::parse_num("8NOPEK");
        assert_eq!(wrong, None);
    }
}

mod note_tests {
    #[test]
    fn note_period() {
        let freq = super::note::get_freq(60.0);
        assert_eq!(freq, 1.0)
    }
}
