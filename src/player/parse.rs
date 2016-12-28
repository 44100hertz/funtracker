use player::base32;
use player::song::Song;
use player::channel::Chan;

/// parse field string with syntax N-O cXXXX
pub fn parse_line(song: &mut Song, line: &str) {
    let fields = line.split("|")
        .map(|s| s.trim());

    let mut i = 0;
    for field in fields {
        let mut words = field.split_whitespace();
        let ref mut chan = song.chans[i];
        if let Some(word) = words.next() { set_note(word, chan) }
        if let Some(word) = words.next() { set_command(word, chan) }
        i = i + 1;
    }
}

pub fn set_note(note: &str, chan: &mut Chan) {
    let chars = note.chars().collect::<Vec<char>>();
    if chars.len() < 3 { return };

    match chars[0] {
        c @ 'A'...'G' => chan.note = parse_note(chars),
//        i @ '0'...'9' => chan.set_inst(),
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
pub fn set_command(command: &str, chan: &mut Chan) {
    fn d_num(value: &str, default: f64) -> f64 {
        match parse_num(value) {
            Some(v) => v,
            None => default,
        }
    }

    if command.len()==0 {return};
    let (id, v) = command.split_at(1);
    match id {
        "2" => chan.samp_off  = d_num(v, 0.0),
        "3" => chan.samp_len  = d_num(v, 0.0),
        "6" => chan.samp_rate = d_num(v, 16000.0),
        "8" => chan.wave      = d_num(v, 0.0),
        "9" => chan.phase     = d_num(v, 0.0),
        "A" => chan.volume    = d_num(v, 0.5),
        "N" => chan.note      = d_num(v, 48.0),
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
