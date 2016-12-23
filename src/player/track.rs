pub struct Field {
    pub note: Option<i32>,
    pub command: Option<char>,
    pub value: Option<f32>,
}

pub struct Track {
    pattern: Vec<Field>,
}
