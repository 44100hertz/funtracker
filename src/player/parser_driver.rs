use std::fs::File;
use std::io::Read;
use player::song::Song;
use player::parser;

pub fn read_track_from_dir() -> Song {
    let sequence = match File::open("test_song/sequence") {
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

