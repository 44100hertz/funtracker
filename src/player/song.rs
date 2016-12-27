use player::note;
use player::command;

pub struct Channel {
    pub samp_off: f64,
    pub samp_len: f64,
    pub samp_rate: f64,
    pub wave: f64,
    pub phase: f64,
    pub volume: f64,
    pub note: f64,
}

impl Channel {
    fn new() -> Channel {
        Channel {
            samp_off: 0.0,
            samp_len: 73.0,
            samp_rate: 0.0,
            wave: 0.0,
            phase: 0.0,
            volume: 0.5,
            note: 0.0,
        }
    }
}

pub struct Field {
    pub note: Option<i32>,
    pub command: Option<String>,
}

pub struct Song {
    channels: Vec<Channel>,
    track: Vec<Vec<Field>>,
    bpm: f64,
    tick_countdown: f64,
    point_period: f64,
    field: usize,
    samples: Vec<u8>,
}

impl Song {
    pub fn new(seq: Vec<Vec<Field>>, samples: Vec<u8>) -> Song {
        Song {
            channels: {
                let mut tmp = Vec::new();
                for _ in &seq[0] {tmp.push(Channel::new());}
                tmp
            },

            track: seq,
            bpm: 120.0,
            tick_countdown: 0.0,
            point_period: (1.0 / 48000.0),
            field: 0,
            samples: samples,
        }
    }

    fn tick(&mut self) {
        self.tick_countdown += 60.0 / self.bpm;

        for i in 0..self.track[self.field].len() {
            if let Some(note) = self.track[self.field][i].note
            { self.channels[i].note = note as f64 }
            if let Some(ref c) = self.track[self.field][i].command
            { command::set(&mut self.channels[i], c) }
        }
        self.field += 1;
    }

    pub fn get_point(&mut self) -> f32 {
        // Tick management
        if self.tick_countdown < 0.0 { self.tick(); }
        self.tick_countdown -= self.point_period;

        // Mix audio
        let mut mix: f64 = 0.0;
        for c in &mut self.channels {
            let phase_ratio = self.point_period * c.samp_rate;
            let phase_offset = note::get_freq(c.note) * phase_ratio;
            c.phase = (c.phase + phase_offset) % c.samp_len;
            let samp_index = (c.phase + c.samp_off) as usize;
            c.wave = self.samples[samp_index] as f64 / 255.0;
            mix += c.wave * c.volume;
        }

        mix as f32
    }
}
