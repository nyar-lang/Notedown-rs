use notedown_ast::{Context, NotedownMeta, ToHTML};
use std::fs::{read_to_string, File, Metadata};

use std::{io::Write, path::Path};
use walkdir::{DirEntry, Error};

pub trait ZolaBackend {
    fn parse_source(&mut self, path_from: &str) -> Result<(), std::io::Error>;
    fn write_target(&self) -> Result<(), std::io::Error>;
}
// impl ZolaBackend for Context {
// fn parse_source(&mut self, path_from: &str) -> Result<(), std::io::Error> {
// let r = read_to_string(path_from)?;
// self.parse(&r);
// return Ok(());
// }
// fn write_target(&self) -> Result<(), std::io::Error> {
// let mut file = match self.get_name() {
// None => File::create("Untitled")?,
// Some(s) => File::create(s)?,
// };
// let mut cfg = HTMLConfig::default();
// cfg.target = String::from("zola");
// file.write_all(self.to_html_with(&cfg).as_bytes())?;
// return Ok(());
// }
// }

#[test]
fn parse() {
    let mut c = Context::default();
    c.parse(
        r#"
    \tags: a, b, c
    \cats: e | f | g
    # a



    "#,
    );
    println!("{}", c.to_html())
}

pub fn parse_file(dir: DirEntry) -> Context {
    let mut cfg = Context::default();
    if let Ok(meta) = dir.metadata() {
        if let Ok(time) = meta.created() {
            cfg.meta.created_time = Some(time);
        }
        if let Ok(time) = meta.modified() {
            cfg.meta.modified_time = Some(time);
        }
    }
    cfg.meta.file_path = Some(dir.into_path());
    println!("path: {:?}", cfg.meta.file_path);

    return cfg;
}
