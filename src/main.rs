extern crate sdl2;

mod player;
use player::track::Field;
use player::channel::Channel;
use player::song::Song;

mod audio;
use audio::Audio;

fn main() {
    let sdl_context = sdl2::init()
        .expect("failed to init sdl2");

    let song = Song {
        track: vec![
            Field { note: Some(48), command: None, value: None },
            Field { note: Some(50), command: None, value: None },
            Field { note: Some(52), command: None, value: None },
            Field { note: Some(54), command: None, value: None },
        ],
        channels: vec![
            Channel::new()
        ],
    };

    let mut out = Audio::new(sdl_context, 48000);
    out.play_song(song);
}
