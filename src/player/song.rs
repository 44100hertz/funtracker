use player::parser;
use player::note;

pub struct Field {
    pub note: Option<i32>,
    pub command: Option<char>,
    pub value: Option<f64>,
}

struct Channel {
    samp_len: f64,
    samp_rate: f64,
    wave: f64,
    phase: f64,
    volume: f64,
    note: f64,
}

pub struct Song {
    track: Vec<Field>,
    channels: Vec<Channel>,
    bpm: f64,
    tick_countdown: f64,
    point_period: f64,
    field: usize,
}

impl Channel {
    fn new() -> Channel {
        Channel {
            samp_len: 200.0,
            samp_rate: 440.0,
            wave: 0.0,
            phase: 0.0,
            volume: 1.0,
            note: 0.0,
        }
    }

    pub fn get_point(&mut self) {
        let period = note::get_period(self.note) * self.samp_rate;
        self.phase = (self.phase + period) % (self.samp_len);
        self.wave = if self.phase > self.samp_len / 2.0 {1.0} else {-1.0}
    }
}

impl Song {
    pub fn new(seq: &str, num_channels: i32) -> Song {
        Song {
            track: parser::parse_seq(seq),
            channels: {
                let mut tmp = Vec::new();
                for _ in 0..num_channels {tmp.push(Channel::new());}
                tmp
            },
            bpm: 120.0,
            tick_countdown: 0.0,
            point_period: (1.0 / 48000.0),
            field: 0,
        }
    }


    fn tick(&mut self) {
        self.tick_countdown += 60.0 / self.bpm;

        let note = self.track[self.field].note;
        if note.is_some() {self.channels[0].note = note.unwrap() as f64}
        self.field += 1;
    }

    pub fn get_point(&mut self) -> f32 {
        // Tick management
        if self.tick_countdown < 0.0 { self.tick(); }
        self.tick_countdown -= self.point_period;

        // Update channel states
        for c in &mut self.channels {
            c.get_point();
        }

        // Make final mix
        let mut mix: f64 = 0.0;
        for c in &mut self.channels {
            mix += c.wave * c.volume;
        }

        mix as f32
    }
}
