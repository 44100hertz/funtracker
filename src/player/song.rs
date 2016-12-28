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
pub struct Song<'a> {
    pub track: &'a Track,
    pub chan: Vec<Chan>,
    pub bpm: f64,
    tick_countdown: f64,
    point_period: f64,
    field: usize,
}

impl<'a> Song<'a> {
    pub fn new(track: &Track, num_chans: usize) -> Song {
        Song {
            track: track,
            chan: {
                let mut tmp = Vec::new();
                for _ in 1..num_chans {tmp.push(Chan::new());}
                tmp },
            bpm: 0.0,
            tick_countdown: 0.0,
            point_period: (1.0 / 48000.0),
            field: 0,
        }
    }

    fn tick(&mut self) {
        self.tick_countdown += 60.0 / self.bpm;

        let fields = self.track.seq[self.field]
            .split("|")
            .map(|s| s.trim());

        let mut i: usize = 0;
        for field in fields {
            let mut words = field.split_whitespace();
            if let Some(word) = words.next() {
                parse::apply_note(word, self, i)
            };
            if let Some(word) = words.next() {
                parse::apply_command(word, self, i)
            };
            i = i + 1;
        }

        self.field += 1;
    }

    pub fn get_point(&mut self) -> f32 {
        // Tick management
        if self.tick_countdown < 0.0 { self.tick(); }
        self.tick_countdown -= self.point_period;

        // Mix audio
        let mut mix: f64 = 0.0;
        for c in &mut self.chan {
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
            c.wave = self.track.samp[samp_index] as f64 / 255.0;
            // Mix the channel's wave
            mix += c.wave * c.volume.max(0.0);
        }

        mix as f32
    }

    pub fn apply_inst(&mut self, chan: usize, part: usize) {
        parse::apply_command(
            &self.track.inst[self.chan[chan].inst][part],
            self,
            chan,
        )
    }
}
