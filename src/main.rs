mod song;
mod channel;
mod output;

use output::SoundPlayer;

fn main() {
    let mut sng = song::Song {
        channels: vec![
            channel::Channel {
                samp_off: 0,
                samp_len: 20,
                samp_rate: 0f32,
                wave: 0f32,
                phase: 0f32,
                volume: 0.5f32,
                note: 0,
            },
        ]
    };

    let mut out = output::rawpcm::RawPCM::open("testout.pcmf32");

    for _ in 1..100 {
        out.play(sng.get_chunk(256));
    }
}
