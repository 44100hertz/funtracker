extern crate sdl2;

mod player;
use player::song::Song;

mod audio;
use audio::Audio;

use std::fs::File;
use std::io::Read;

fn main() {
    let sdl_context = sdl2::init()
        .expect("failed to init sdl2");

    let mut trackfile = File::open("res/parse-test1")
        .expect("could not open test parsing file");
    let mut s = String::new();
    trackfile.read_to_string(&mut s)
        .expect("error reading file");

    let song = Song::new(&s, 1);

    let mut out = Audio::new(sdl_context, 48000);
    out.play_song(song);
}
