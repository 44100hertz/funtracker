mod song;
mod channel;

fn main() {
    let mut sng = song::Song {
        channels: Vec::new(),
    };

    sng.channels.push(
        channel::Channel {
            samp_off: 0,
            samp_len: 20,
            samp_rate: 0f32,
            wave: 0f32,
            phase: 0f32,
            volume: 1f32,
            note: 0,
        }
    );

    for _ in 1..100 {
        println!("{}", sng.update());
    }
}
