pub mod rawpcm;

pub trait SoundPlayer {
    fn play(&mut self, data: f32);
}
