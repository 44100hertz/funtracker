pub struct Chan {
    pub inst: usize,
    pub samp_off: f64,
    pub samp_len: f64,
    pub samp_rate: f64,
    pub wave: f64,
    pub phase: f64,
    pub volume: f64,
    pub note: f64,
}

impl Chan {
    pub fn new() -> Chan {
        Chan {
            inst: 0,
            samp_off: 0.0,
            samp_len: 0.0,
            samp_rate: 0.0,
            wave: 0.0,
            phase: 0.0,
            volume: 0.0,
            note: 0.0,
        }
    }
}
