use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use player::song::Song;
use player::sequence;

pub fn read_track_from_dir(pathstr: &str) -> Song {
    let mut path = PathBuf::from(pathstr);

    path.push("sequence");
    let sequence = match File::open(&path) {
        Ok(mut file) => {
            let mut s = String::new();
            file.read_to_string(&mut s)
                .expect("can't read sequence");
            s.lines()
                .collect::<Vec<String>>()
        },
        Err(_) => Vec::new(),
    };

    path.pop();
    path.push("instruments");
    let insts = match File::open(&path) {
        Ok(mut file) => {
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();
            s.lines()
        }
        Err(_) => Vec::new(),
    };

    path.pop();
    path.push("samples.raw");
    let mut samples = Vec::new();
    if let Ok(mut file) = File::open(&path) {
        file.read_to_end(&mut samples)
            .expect("error reading file");
    };

    Song::new(sequence, samples, insts)
}
