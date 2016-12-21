mod player;

fn main() {
    let mut chan = player::channel::Channel {
        samp_off: 0,
        samp_len: 20,
        samp_rate: 0f32,
        wave: 0f32,
        phase: 0f32,
        volume: 0f32,
        note: 0,
    };

    for _ in 1..100 {
        chan.update();
    }
}
