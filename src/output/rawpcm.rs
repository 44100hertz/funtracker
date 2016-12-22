use std::mem;
use std::io::prelude::*;
use std::fs::File;

use output::SoundPlayer;

pub struct RawPCM {
    file: File,
}

impl SoundPlayer for RawPCM {
    fn play(&mut self, data: f32) {
        unsafe {
            let rawdata = mem::transmute::<f32, [u8; 4]>(data);
            self.file.write_all(&rawdata)
                .expect("could not write file");
        }
    }
}

impl RawPCM {
    pub fn open(filename: &str) -> RawPCM {
        let f = File::create(filename)
            .expect("could not create file");
        RawPCM {
            file: f
        }
    }
}