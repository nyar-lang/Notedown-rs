extern crate walkdir;

mod traits;
mod utils;

use crate::{traits::file_to_zola, utils::filter_files};
use notedown_ast::{NotedownBackend, NotedownConfig, GLOBAL_CONFIG};
use std::{fs, path::PathBuf};

fn reinitialize() {
    let mut x = GLOBAL_CONFIG.lock().unwrap();
    *x = NotedownConfig { tab_size: 2, target: NotedownBackend::Zola };
}

#[cfg(not(test))]
fn main() {
    reinitialize();
    let files = filter_files("./source");
    let dir = PathBuf::from("./content");
    fs::create_dir_all(&dir).unwrap();
    for file in files {
        if let Err(e) = file_to_zola(file, &dir) {
            println!("Error: {}", e)
        }
    }
}

#[cfg(test)]
fn main() {
    reinitialize();
    let files = filter_files("./projects");
    let dir = PathBuf::from("./target/notedown/");
    fs::create_dir_all(&dir).unwrap();
    for file in files {
        if let Err(e) = file_to_zola(file, &dir) {
            println!("Error: {}", e)
        }
    }
}
