extern crate sdl2;
use self::sdl2::audio::{AudioSpecDesired};
use std::time::Duration;

mod player;
use player::song::Song;
use player::channel::Channel;

mod output;
use output::Output;

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

    let out = Output {
        song: sng,
    };

    let sdl_context = sdl2::init()
        .expect("failed to init sdl2");
    let audio_subsystem = sdl_context.audio()
        .expect("failed to init sdl2 audio");

    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(1),
        samples: None,
    };

    let device = audio_subsystem.open_playback(
        None, &desired_spec, |_| {out})
        .expect("could not open playback");
    device.resume();

    std::thread::sleep(Duration::from_millis(2000));
}
