use player::channel::Channel;

pub struct Song {
    pub channels: Vec<Channel>,
}

impl Song {
    pub fn new() -> Song {
        Song {
            channels: vec![Channel::new()],
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
