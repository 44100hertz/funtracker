pub struct Field {
    pub note: Option<i32>,
    pub command: Option<char>,
    pub value: Option<f32>,
}

pub struct Channel {
    pub samp_off: u32,
    pub samp_len: u32,
    pub samp_rate: f32,

    pub wave: f32,
    pub phase: f32,
    pub volume: f32,
    pub note: u32,
}

pub struct Song {
    pub track: Vec<Field>,
    pub channels: Vec<Channel>,
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

impl Song {
    pub fn new() -> Song {
        Song {
            track: Vec::new(),
            channels: Vec::new(),
        }
    }

    pub fn get_point(&mut self) -> f32 {
        // Update channel states
        for c in &mut self.channels {
            c.get_point();
        }

        // Make final mix
        let mut mix: f32 = 0f32;
        for c in &mut self.channels {
            mix += c.wave * c.volume;
        }

        mix
    }
}
