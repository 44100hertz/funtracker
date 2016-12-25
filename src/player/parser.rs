use player::base32;
use player::song::Field;

pub fn parse_seq(track: &str) -> Vec<Vec<Field>> {
    let mut parsed = Vec::new();
    for line in track.split("\n") { parsed.push(parse_line(line)); }
    parsed
}

pub fn parse_line(line: &str) -> Vec<Field> {
    let mut parsed = Vec::new();
    for field in line.split("|") {
        match parse_field(field.trim()) {
            Ok(result) => parsed.push(result),
            Err(err) => println!("{}", err),
        }
    }
    parsed
}

/// parse field with syntax N-O cXXXX
pub fn parse_field(field: &str) -> Result<Field, String> {
    if field.len() != 9 {
        return Err(format!("Field too short, ignored: {}", field));
    }

    let note = parse_note(&field[0..3]);
    let command = base32::char_to_base32(field.as_bytes()[4] as char);

    let value = match *&field[5..9].parse::<f64>() {
        Ok(val) => val,
        Err(_) => 0.0,
    };

    Ok(Field {
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
