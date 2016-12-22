extern crate sdl2;
use self::sdl2::audio::{AudioCallback};

use player::song::Song;

pub struct Output {
    pub song: Song,
}

impl AudioCallback for Output {
    type Channel = f32;
    fn callback(&mut self, out: &mut [f32]) {
        for point in out.iter_mut() {
            *point = self.song.get_point();
        }
    }
}

