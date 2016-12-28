use player::note;
use player::channel::Chan;
use player::parse;

// Static data for a single track
pub struct Track {
    pub seq: Vec<String>,
    pub inst: Vec<Vec<String>>,
    pub samp: Vec<u8>,
}

// Live song parameters
pub struct Song {
    chan: Vec<Chan>,
    bpm: f64,
    tick_countdown: f64,
    point_period: f64,
    field: usize,
}

impl Song {
    pub fn new() -> Song {
        Song {
            chan: { let mut tmp = Vec::new();
                    for _ in 1..seq.len() {tmp.push(Chan::new());}
                    tmp
            }
            bpm: 0.0,
            tick_countdown: 0.0,
            point_period: (1.0 / 48000.0),
            field: 0,
            samples: samples,
        }
    }
}

impl Song {
    fn tick(&mut self) {
        self.tick_countdown += 60.0 / self.bpm;

//        for mut t in &mut self.track {
//            parse::parse_line(*self, &mut t);
//        }
        self.field += 1;
    }

    pub fn get_point(&mut self) -> f32 {
        // Tick management
        if self.tick_countdown < 0.0 { self.tick(); }
        self.tick_countdown -= self.point_period;

        // Mix audio
        let mut mix: f64 = 0.0;
        for c in &mut self.chans {
            // Get the ratio between native and chan sample rates;
            // this is the "desired" point period.
            let phase_ratio = self.point_period * c.samp_rate;
            // Adjust the desired point period by the frequency offset
            let phase_offset = note::get_freq(c.note) * phase_ratio;
            // Increase the phase, using a modulo for looping
            // Known error: phase offsets that overflow usize break
            c.phase = (c.phase + phase_offset) % c.samp_len;
            // Add this to the sample offset to find offset within bank
            let samp_index = (c.phase + c.samp_off) as usize;
            // Grab the current phase from this offset
            c.wave = self.samples[samp_index] as f64 / 255.0;
            // Mix the channel's wave
            mix += c.wave * c.volume.max(0.0);
        }

        mix as f32
    }

    pub fn apply_inst(&mut self, chan: usize, part: usize) {
        parse::apply_command(
            self.insts[self.chans[chan].inst][part],
            self,
            chan,
        )
    }
}
