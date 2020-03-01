use notedown_ast::{Context, MissingCommand, NotedownTarget, ToHTML};
use std::fs::File;

use std::{fs, io::Write, panic, path::PathBuf};
use walkdir::DirEntry;

pub trait ZolaBackend {
    fn parse_source(&mut self, path_from: &str) -> Result<(), std::io::Error>;
    fn write_target(&self) -> Result<(), std::io::Error>;
}

pub fn file_to_zola(path: DirEntry, out: &PathBuf) -> Result<(), &'static str> {
    let mut ctx = Context::default();
    ctx.cfg.template = MissingCommand::Zola;
    ctx.cfg.target = NotedownTarget::Zola;
    if let Ok(meta) = path.metadata() {
        if let Ok(time) = meta.created() {
            ctx.meta.created_time = Some(time);
        }
        if let Ok(time) = meta.modified() {
            ctx.meta.modified_time = Some(time);
        }
    }
    ctx.meta.file_path = Some(path.clone().into_path()); //FIXME: remove clone
    ctx.meta.file_name = Some(path.path().file_stem().unwrap().to_str().unwrap().to_string());
    ctx.meta.title = ctx.meta.file_name.clone();
    ctx.parse(&fs::read_to_string(path.clone().into_path()).expect("can't read the file"));
    let html = match panic::catch_unwind(|| ctx.to_html()) {
        Ok(s) => s,
        Err(_) => return Err("can not parse this file"),
    };
    let mut out = out.clone();
    out.push(&ctx.meta.file_name.unwrap());
    out.set_extension("md");
    let mut file = File::create(&out).expect("can not create file");
    file.write_all(html.as_bytes()).expect("can not write to file");
    println!("Build: {:?}", out);
    Ok(())
}
