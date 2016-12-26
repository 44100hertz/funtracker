use std::path::PathBuf;
use std::io::{Read, Write};
use player::song::Song;
use player::parse;

extern crate memmap;
use self::memmap::{Mmap, Protection};

pub fn track_from_dir(pathstr: &str) {
    let mut path = PathBuf::from(pathstr);

    path.push("sequence");
    let patt = Mmap::open_path(&path, Protection::Read).unwrap();

    path.pop();
    path.push("samples.raw");
    let samples = Mmap::open_path(&path, Protection::Read).unwrap();
}
