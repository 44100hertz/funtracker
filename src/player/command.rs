use player::parse::parse_num as p;
use player::song::Channel;

/// Set a parameter
pub fn set(chan: &mut Channel, command: &str) {
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

/// Try to parse out a value, default if not
pub fn d_num(value: &str, default: f64) -> f64 {
    match p(value) {
        Some(v) => v,
        None => default,
    }
}
