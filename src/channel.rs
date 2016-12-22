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
    pub fn update(&mut self) {
        self.phase = (self.phase + 1f32) % (self.samp_len as f32);
        // Make a square wave
        self.wave =
            if self.phase > (self.samp_len / 2) as f32 {1f32}
            else {-1f32};
    }
}
