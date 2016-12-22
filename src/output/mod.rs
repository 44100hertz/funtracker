pub mod rawpcm;

trait SoundPlayer {
    fn open(&mut self);
    fn play(&mut self, data: f32);
}
