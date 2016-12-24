use player::base32;
use player::song::Field;

pub fn parse_seq(track: &str) -> Vec<Field> {
    let lines = track.split("\n");
    let mut parsed = Vec::new();
    for field in lines {
        let field_parse = parse_field(field);
        if field_parse.is_ok() {parsed.push(field_parse.unwrap());}
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
    let value = *&field[5..9].parse().ok();

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
        '-' => 0,
        _ => return None,
    };

    let octave = match (bytes[2] as char).to_digit(10) {
        Some(octave) => octave,
        None => return None,
    };

    Some(octave as i32 * 12 + letter_offset + accidental_offset)
}
