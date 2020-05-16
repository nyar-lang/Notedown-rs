use notedown_ast::{parse, ToHTML};

use std::{fs::File, io::prelude::*};

#[test]
fn test_math() {
    let mut file = File::create("tests/commands.md").unwrap();
    let f = parse(include_str!("commands.note"));
    // println!("{:#?}", f);
    file.write_all(f.to_html().as_bytes()).unwrap();
}

#[test]
fn ready() {
    println!("it, works!")
}