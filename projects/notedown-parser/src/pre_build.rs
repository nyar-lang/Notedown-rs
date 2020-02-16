use pest_generator::derive_parser;
use std::{fs::File, io::prelude::*, path::Path};

#[test]
#[ignore]
fn gen_parser() {
    gen_note_down();
}

pub fn gen_note_down() {
    let pest = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./note_down.pest"));
    let rs = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./src/note_down.rs"));

    let derived = {
        let path = pest.to_string_lossy();
        let pest = quote! {
            #[grammar = #path]
            pub struct NoteDownParser;
        };
        derive_parser(pest, false)
    };
    let mut file = File::create(rs).unwrap();
    let out = format!("pub struct NoteDownParser;{}", derived);
    writeln!(file, "{}", out).unwrap();
}

pub fn gen_note_text() {
    let pest = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./note_text.pest"));
    let rs = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./src/note_text.rs"));

    let derived = {
        let path = pest.to_string_lossy();
        let pest = quote! {
            #[grammar = #path]
            pub struct NoteTextParser;
        };
        derive_parser(pest, false)
    };
    let mut file = File::create(rs).unwrap();
    let out = format!("pub struct NoteTextParser;{}", derived);
    writeln!(file, "{}", out).unwrap();
}
