use std::fs::{remove_file, File};
use std::io::prelude::*;

fn main() {
    let s: String = std::fs::read_to_string("tiles-data-backup.txt").expect("Unable to read file");

    let _ = remove_file("tiles-data.txt");
    let mut file = File::create("tiles-data.txt").expect("Error encountered while creating file!");

    file.write_all((&s[..]).as_bytes())
        .expect("Error while writing to file");
}
