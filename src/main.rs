extern crate sdl2;

mod player;
use player::song::Song;
use player::channel::Channel;

mod audio;
use audio::Audio;

fn main() {
    let sng = Song::new();

    let sdl_context = sdl2::init()
        .expect("failed to init sdl2");

    let mut out = Audio::new(sdl_context, 48000);
    out.play_song(sng);
}
