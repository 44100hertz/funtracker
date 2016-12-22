use channel::Channel;

pub struct Song {
    pub channels: Vec<Channel>,
}

impl Song {
    pub fn update(&mut self) -> f32 {
        // Update channel states
        for c in &mut self.channels {
            c.update();
        }

        // Make final mix
        let mut mix: f32 = 0f32;
        for c in &mut self.channels {
            mix += c.wave * c.volume;
        }

        mix
    }
}
