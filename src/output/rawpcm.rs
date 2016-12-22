use std::mem;
use std::io::prelude::*;
use std::fs::File;

use output::SoundPlayer;

struct RawPCM {
    file: File,
}

impl SoundPlayer for RawPCM {
    fn open(&mut self) {
        let f = File::create("testout.pcm32f")
            .expect("could not create file");
        self.file = f;
    }

    fn play(&mut self, data: f32) {
        unsafe {
            let rawdata = mem::transmute::<f32, [u8; 4]>(data);
            self.file.write_all(&rawdata)
                .expect("could not write file");
        }
    }
}
