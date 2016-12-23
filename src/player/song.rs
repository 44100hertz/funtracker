use player::channel::Channel;
use player::track::Field;

pub struct Song {
    pub track: Vec<Field>,
    pub channels: Vec<Channel>,
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
