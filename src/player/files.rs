use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use player::song::Track;

pub fn read_track_from_dir(pathstr: &str) -> Track {
    let mut path = PathBuf::from(pathstr);

    path.push("parse");
    let seq = match File::open(&path) {
        Ok(mut file) => {
            let mut s = String::new();
            file.read_to_string(&mut s)
                .expect("can't read parse");
            split_song(s)
        },
        Err(_) => Vec::new(),
    };

    path.pop();
    path.push("instruments");
    let inst = match File::open(&path) {
        Ok(mut file) => {
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();
            split_insts(s)
        }
        Err(_) => Vec::new(),
    };

    path.pop();
    path.push("samples.raw");
    let mut samp = Vec::new();
    if let Ok(mut file) = File::open(&path) {
        file.read_to_end(&mut samp)
            .expect("error reading file");
    };

    Track {
        seq: seq,
        inst: inst,
        samp: samp,
    }
}

pub fn split_song(s: String) -> Vec<String> {
    s.lines()
        .map(|s| s.to_owned())
        .collect()
}

pub fn split_insts(s: String) -> Vec<Vec<String>> {
    s.lines()
        .map(|s| s.split("|")
             .map(|s| s.to_owned())
             .collect())
        .collect()
}
