pub fn get_freq(num: i32) -> f32 {
    440f32 * 2f32.powf(((num - 60) as f32 / 12f32))
}

pub fn get_period(num: i32) -> f32 {
    (1.0/440.0) as f32 * 2f32.powf(-((num - 60) as f32 / 12f32))
}
