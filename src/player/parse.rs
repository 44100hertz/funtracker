use player::base32;
use player::song::Field;

/// Sanitize a sequence block and return a usable internal sequence
pub fn parse_seq(track: String) -> Vec<Vec<Field>> {
    track.lines()
        .map(parse_line)
        .collect::<Vec<Vec<Field>>>()
}

/// Parse a single line of channels
pub fn parse_line(line: &str) -> Vec<Field> {
    line.split("|")
        .map(parse_field)
        .collect::<Vec<Field>>()
}

/// parse field with syntax N-O cXXXX
pub fn parse_field(field: &str) -> Field {
    let mut words = field.trim().split_whitespace();
    let note = match words.next() {
        Some(word) => parse_note(word),
        None => parse_note(&field),
    };
    let command = match words.next() {
        Some(s) => Some(base32::sanitize(s)),
        None => None,
    };

    Field {
        note: note,
        command: command,
    }
}

/// Return a midi note from a string e.g. "C-4"
pub fn parse_note(note: &str) -> Option<i32> {
    let chars = note.chars().collect::<Vec<char>>();
    if chars.len() < 3 { return None };

    let letter_offset = match chars[0] {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return None,
    };

    let accidental_offset = match chars[1] {
        '#' => 1,
        _ => 0,
    };

    let octave = match chars[2].to_digit(10) {
        Some(o) => o as i32 - 4,
        None => 0,
    };

    Some(octave*12 + letter_offset + accidental_offset)
}

/// Parse a clean number with format:
/// "8" = 8.0, "8K" = 8,000, "8M" = 0.008, etc.
pub fn parse_num(numstr: &str) -> Option<f64> {
    fn np(s: &str, mult: f64) -> Option<f64> {
        match *&s[0..s.len()-1].parse::<f64>() {
            Ok(num) => Some(num * mult),
            Err(_) => None,
        }
    }

    let s = numstr.trim();

    let b = s.as_bytes();
    match b[b.len()-1] {
        b'0'...b'9' => s.parse().ok(),
        b'K' => np(s, 1000.0),
        b'H' => np(s, 100.0),
        b'C' => np(s, 0.01),
        b'M' => np(s, 0.001),
        _ => None,
    }
}
