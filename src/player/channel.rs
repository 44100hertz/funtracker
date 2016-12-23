pub struct Channel {
    pub samp_off: u32,
    pub samp_len: u32,
    pub samp_rate: f32,

    pub wave: f32,
    pub phase: f32,
    pub volume: f32,
    pub note: u32,
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            samp_off: 0,
            samp_len: 0,
            samp_rate: 0f32,
            wave: 0f32,
            phase: 0f32,
            volume: 1f32,
            note: 0,
        }
    }

    pub fn get_point(&mut self) {
        self.phase = (self.phase + 1f32) % (self.samp_len as f32);
        // Make a square wave
        self.wave =
            if self.phase > (self.samp_len / 2) as f32 {1f32}
            else {-1f32};
    }
}
