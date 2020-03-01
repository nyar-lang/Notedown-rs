use notedown_ast::{parse, ToHTML};

use std::{fs::File, io::prelude::*};

#[test]
fn test_math() {
    let mut file = File::create("tests/normal/part1.md").unwrap();
    let f = parse(include_str!("part1.note"));
    // println!("{:#?}", f);
    file.write_all(f.to_html().as_bytes()).unwrap();
}

#[test]
fn test_style() {
    let mut file = File::create("tests/normal/style.md").unwrap();
    let f = parse(include_str!("style.note"));
    // println!("{:#?}", f);
    file.write_all(f.to_html().as_bytes()).unwrap();
}

#[test]
fn test_asterisk() {
    let mut file = File::create("tests/normal/asterisk.md").unwrap();
    let f = parse(include_str!("asterisk.note"));
    // println!("{:#?}", f);
    file.write_all(f.to_html().as_bytes()).unwrap();
}

#[test]
fn test_command() {
    let mut file = File::create("tests/normal/commands.md").unwrap();
    let f = parse(include_str!("commands.note"));
    // println!("{:#?}", f);
    file.write_all(f.to_html().as_bytes()).unwrap();
}
