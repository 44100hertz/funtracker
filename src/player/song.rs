use player::parser;

pub struct Field {
    pub note: Option<i32>,
    pub command: Option<char>,
    pub value: Option<f64>,
}

struct Channel {
    samp_len: f64,
    wave: f64,
    phase: f64,
    volume: f64,
    note: f64,
}

pub struct Song {
    track: Vec<Field>,
    channels: Vec<Channel>,
    tick_countdown: f64,
    point_period: f64,
}

impl Channel {
    fn new() -> Channel {
        Channel {
            samp_len: 200.0,
            wave: 0.0,
            phase: 0.0,
            volume: 1.0,
            note: 0.0,
        }
    }

    pub fn get_point(&mut self) {
        self.phase = (self.phase + 1.0) % (self.samp_len);
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
            tick_countdown: 0.0,
            point_period: (1.0 / 48000.0),
        }
    }


    fn tick(&mut self) {
        self.tick_countdown += 0.5;
        println!("tick");
    }

    pub fn get_point(&mut self) -> f32 {
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
