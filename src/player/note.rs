pub fn get_freq(num: i32) -> f64 {
    440.0 * 2f64.powf(((num - 60) as f64 / 12.0))
}

pub fn get_period(num: i32) -> f64 {
    1.0/440.0 * 2f64.powf(-((num - 60) as f64 / 12.0))
}
