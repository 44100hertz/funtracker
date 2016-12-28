use player::song::Song;

pub fn apply_note(note: &str, song: &mut Song, chan: usize) {
    let chars = note.chars().collect::<Vec<char>>();
    if chars.len() < 3 { return };

    match chars[0] {
        'A'...'G' =>
            song.chan[chan].note = parse_note(chars),
        i @ '0'...'9' =>
            song.apply_inst(chan, i.to_digit(10).unwrap() as usize),
        _ => {},
    }
}

pub fn parse_note(chars: Vec<char>) -> f64 {
    let mut offset = 0;
    offset += match chars[0] {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => 0,
    };
    offset += match chars[1] {
        '#' => 1,
        _ => 0,
    };
    if let Some(octave) = chars[2].to_digit(10) {
        offset += (octave as i32 - 4) * 12;
    }
    offset as f64
}

/// Set a parameter
pub fn apply_command(command: &str, song: &mut Song, chan: usize) {
    fn d_num(value: &str, default: f64) -> f64 {
        match parse_num(value) {
            Some(v) => v,
            None => default,
        }
    }

    if command.len()==0 {return};
    let (id, v) = command.split_at(1);
    let ref mut c = song.chan[chan];
    match id {
        "2" => c.samp_off  = d_num(v, 0.0),
        "3" => c.samp_len  = d_num(v, 0.0),
        "6" => c.samp_rate = d_num(v, 16000.0),
        "8" => c.wave      = d_num(v, 0.0),
        "9" => c.phase     = d_num(v, 0.0),
        "A" => c.volume    = d_num(v, 0.5),
        "N" => c.note      = d_num(v, 48.0),
        _ => {}
    }
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
