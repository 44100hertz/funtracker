use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use player::song::Song;
use player::parser;

pub fn read_track_from_dir(pathstr: &str) -> Song {
    let mut path = PathBuf::from(pathstr);
    path.push("sequence");

    let sequence = match File::open(&path) {
        Ok(mut seq_file) => {
            let mut seq_str = String::new();
            seq_file.read_to_string(&mut seq_str)
                .expect("error reading file");
            parser::parse_seq(&seq_str)
        },
        Err(_) => Vec::new(),
    };

    Song::new(sequence, 1)
}
