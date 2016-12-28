pub mod channel;
pub mod song;
pub mod note;
pub mod base32;
pub mod sequence;
pub mod files;

#[cfg(test)]
mod parse_tests {
    use super::sequence;
    use super::base32;
    #[test]
    fn middle_c() {
        let note = sequence::parse_note("C-4".chars().collect());
        assert_eq!(note, 0.0)
    }

    #[test]
    fn a_sharp_0() {
        let note = sequence::parse_note("A#0".chars().collect());
        assert_eq!(note, -38.0)
    }

    #[test]
    fn invalid_note() {
        let note = sequence::parse_note("rip".chars().collect());
        assert_eq!(note, 0.0)
    }

    #[test]
    fn base32_zero() {
        let zero = base32::char_to_base32('o');
        assert_eq!(zero, Some('0'))
    }

    #[test]
    fn parse_kilo() {
        let kilo = sequence::parse_num("1K");
        assert_eq!(kilo, Some(1000.0));
    }

    #[test]
    fn parse_kilo_decimal() {
        let kilo = sequence::parse_num("1.1K");
        assert_eq!(kilo, Some(1100.0))
    }

    #[test]
    fn parse_bad() {
        let wrong = sequence::parse_num("8NOPEK");
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
