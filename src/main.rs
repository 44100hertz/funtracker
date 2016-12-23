extern crate sdl2;

mod player;
use player::song::Song;
use player::channel::Channel;

mod audio;
use audio::Audio;

fn main() {
    let sng = Song {
        channels: vec![
            Channel {
                samp_off: 0,
                samp_len: 200,
                samp_rate: 0f32,
                wave: 0f32,
                phase: 0f32,
                volume: 0.5f32,
                note: 0,
            },
        ]
    };

    let sdl_context = sdl2::init()
        .expect("failed to init sdl2");

    let mut out = Audio::new(sdl_context, 48000);
    out.play_song(sng);
}
