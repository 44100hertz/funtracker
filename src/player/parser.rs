use player::base32;

pub struct Field {
    pub note: Result<i32, ()>,
    pub command: char,
    pub value: f32,
}

/// parse field with syntax N-O cXXXX
pub fn parse_field(field: &str) -> Field {
    let note = parse_note(&field[0..3]);
    let command =
        base32::char_to_base32(field.as_bytes()[4] as char);
    let value: f32 = *&field[5..9].parse()
        .expect("cannot parse field value");

    Field {
        note: note,
        command: command,
        value: value,
    }
}

/// Return a midi note from a string e.g. "C-4"
pub fn parse_note(note: &str) -> Result<i32, ()> {
    let bytes = note.as_bytes();

    let letter_offset = match bytes[0] as char {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return Err(()),
    };

    let accidental_offset = match bytes[1] as char {
        '#' => 1,
        _ => 0,
    };

    let octave = (bytes[2] as char).to_digit(10)
        .expect("octave not a char");

    Ok(octave as i32 * 12 + letter_offset + accidental_offset)
}
