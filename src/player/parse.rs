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
    let mut words = field.split(" ");
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
    let bytes = note.as_bytes();
    if bytes.len() < 3 { return None };

    let letter_offset = match bytes[0] as char {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return None,
    };

    let accidental_offset = match bytes[1] as char {
        '#' => 1,
        _ => 0,
    };

    let octave = match (bytes[2] as char).to_digit(10) {
        Some(octave) => octave,
        None => 4,
    };

    Some(octave as i32 * 12 + letter_offset + accidental_offset)
}

/// Parse a clean number with format:
/// "8" = 8.0, "8K" = 8,000, "8M" = 0.008, etc.
pub fn parse_num(numstr: &str) -> Option<f64> {
    let trimstr = numstr.trim();
    let last = trimstr.as_bytes()[trimstr.len()-1] as char;

    match last {
        '0'...'9' => trimstr.parse().ok(),
        'K' => num_part(trimstr, 1000.0),
        'H' => num_part(trimstr, 100.0),
        'C' => num_part(trimstr, 0.01),
        'M' => num_part(trimstr, 0.001),
        _ => None,
    }
}

/// Helper function for the above
/// Parse everything but the last digit of a number
/// return a number times multiplier if it parses
fn num_part(numstr: &str, mult: f64) -> Option<f64> {
    match *&numstr[0..numstr.len()-1].parse::<f64>() {
        Ok(num) => Some(num * mult),
        Err(_) => None,
    }
}
