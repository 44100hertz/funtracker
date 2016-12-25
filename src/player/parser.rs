use player::base32;
use player::song::Field;

/// Sanitize a sequence block and return a usable internal sequence
pub fn parse_seq(track: &str) -> Vec<Vec<Field>> {
    base32::sanitize(track).lines()
        .map(parse_line)
        .collect::<Vec<Vec<Field>>>()
}

/// Parse a single line of channels
pub fn parse_line(line: &str) -> Vec<Field> {
    line.split("|")
        .filter_map(parse_field)
        .collect::<Vec<Field>>()
}

/// parse field with syntax N-O cXXXX
pub fn parse_field(field: &str) -> Option<Field> {
    let f = field.trim();
    let note = parse_note(&f[0..3]);
    let command = base32::char_to_base32(f.as_bytes()[4] as char);
    let value = match *&f.split_at(5).1 {
        "" => 0.0, // default value
        n @ _ => parse_num(n).unwrap(),
    };

    Some(Field {
        note: note,
        command: command,
        value: value,
    })
}

/// Return a midi note from a string e.g. "C-4"
pub fn parse_note(note: &str) -> Option<i32> {
    let bytes = note.as_bytes();

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
        c @ _ => match *&trimstr[0..trimstr.len()-1].parse::<f64>() {
            Ok(num) => match c {
                'K' => Some(num * 1000.0),
                'H' => Some(num * 100.0),
                'C' => Some(num * (1.0/100.0)),
                'M' => Some(num * (1.0/1000.0)),
                _ => None,
            },
            Err(_) => None,
        }
    }
}
