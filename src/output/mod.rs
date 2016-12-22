pub mod rawpcm;

pub trait SoundPlayer {
    fn play(&mut self, data: Vec<f32>);
}
