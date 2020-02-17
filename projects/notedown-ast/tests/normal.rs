use notedown_ast::{parse, HTMLConfig, ToHTML};

use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_text() {
    let mut file = File::create("tests/part9.md").unwrap();
    let f = parse(include_str!("./part9.notedown"));
    // println!("{:#?}", f);
    file.write_all(f.free().to_html_default().as_bytes())
        .unwrap();
}

#[test]
fn test_style() {
    let mut file = File::create("tests/style.md").unwrap();
    let f = parse(include_str!("./style.notedown"));
    // println!("{:#?}", f);
    file.write_all(f.free().to_html_default().as_bytes())
        .unwrap();
}
