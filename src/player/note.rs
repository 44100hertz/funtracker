pub fn get_freq(num: f64) -> f64 {
    440.0 * 2f64.powf(((num - 60.0) / 12.0))
}

pub fn get_period(num: f64) -> f64 {
    1.0/440.0 * 2f64.powf(((num - 60.0) / 12.0))
}
