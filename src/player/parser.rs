/// Return a midi note from a string e.g. "C-4"
pub fn parse_note(note: &str) -> i32 {
    let mut chars = note.chars();

    let letter_offset = match chars.next().unwrap() {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return -1,
    };

    let accidental_offset = match chars.next().unwrap() {
        '#' => 1,
        _ => 0,
    };

    let octave = chars.next().unwrap().to_digit(10).unwrap() as i32;

    octave * 12 + letter_offset + accidental_offset
}
