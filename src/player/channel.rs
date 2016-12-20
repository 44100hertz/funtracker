struct Channel {
    SampOff: u32,
    SampLen: u32,
    SampRate: f32,

    Wave: f32,
    Phase: f32,
    Volume: f32,
    Note: u32,
}

impl Channel {
    fn update(&mut self, &song: Song) -> f32 {
        // Increase phase by 1 per sample rate
        self.Phase = (self.Phase + 1) % self.SampLen;
        // Make a square wave
        self.Wave = if self.Phase > (self.SampLen / 2) {1} else {-1};
    }
}
