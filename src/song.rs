use channel::Channel;

pub struct Song {
    pub channels: Vec<Channel>,
}

impl Song {
    pub fn get_chunk(&mut self, size: u32) -> Vec<f32> {
        let mut chunk = Vec::new();
        for _ in 0..size {
            chunk.push(self.get_point());
        }
        chunk
    }

    pub fn get_point(&mut self) -> f32 {
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
