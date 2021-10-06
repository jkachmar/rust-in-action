use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    if let Ok(reader) = File::open("chapter_2/readme.md").map(BufReader::new) {
        for line in reader.lines().flatten() {
            println!("{}, ({} bytes long)", line, line.len());
        }
    }
}
