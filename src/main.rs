extern crate sdl2;

mod player;
use player::song::{Song, Field};

mod audio;
use audio::Audio;

fn main() {
    let sdl_context = sdl2::init()
        .expect("failed to init sdl2");

    let mut song = Song::new(1);

    song.track = vec![
        Field { note: Some(48), command: None, value: None },
        Field { note: Some(50), command: None, value: None },
        Field { note: Some(52), command: None, value: None },
        Field { note: Some(54), command: None, value: None },
    ];

    let mut out = Audio::new(sdl_context, 48000);
    out.play_song(song);
}
