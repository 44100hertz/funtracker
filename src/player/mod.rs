pub mod channel;
pub mod song;
pub mod note;
pub mod base32;
pub mod parse;
pub mod files;

#[cfg(test)]
mod parse_tests {
    use super::parse;
    use super::base32;
    #[test]
    fn middle_c() {
        let note = parse::parse_note("C-4".chars().collect());
        assert_eq!(note, 0.0)
    }

    #[test]
    fn a_sharp_0() {
        let note = parse::parse_note("A#0".chars().collect());
        assert_eq!(note, -38.0)
    }

    #[test]
    fn invalid_note() {
        let note = parse::parse_note("rip".chars().collect());
        assert_eq!(note, 0.0)
    }

    #[test]
    fn base32_zero() {
        let zero = base32::char_to_base32('o');
        assert_eq!(zero, Some('0'))
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
        let freq = super::note::get_freq(12.0);
        assert_eq!(freq, 2.0)
    }
}
