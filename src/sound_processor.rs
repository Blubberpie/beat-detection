use hound;
use std::fs::File;
use std::env;
use hound::WavReader;
use std::io::BufReader;

pub fn load_file() -> WavReader<BufReader<File>> {
    let mut path = env::current_dir().unwrap();
    path.push("src");
    path.push("sounds");
    path.push("tetris");
    path.set_extension("wav");

    let mut reader = hound::WavReader::open(path).unwrap();

    reader
}