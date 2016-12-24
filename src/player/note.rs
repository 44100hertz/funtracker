pub fn get_freq(num: f64) -> f64 {
    2f64.powf(((num - 60.0) / 12.0))
}
