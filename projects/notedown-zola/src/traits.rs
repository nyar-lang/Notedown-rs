use chrono::{DateTime, Utc};
use notedown_ast::{Context, ToHTML};
use std::{fs, fs::File, io::Write, panic, path::PathBuf};
use walkdir::DirEntry;

pub fn file_to_zola(path: DirEntry, out: &PathBuf) -> Result<(), &'static str> {
    let mut ctx = Context::default();
    if let Ok(meta) = path.metadata() {
        if let Ok(time) = meta.created() {
            ctx.meta.created_time = Some(DateTime::<Utc>::from(time).naive_utc());
        }
        if let Ok(time) = meta.modified() {
            ctx.meta.modified_time = Some(DateTime::<Utc>::from(time).naive_utc());
        }
    }
    ctx.meta.file_path = Some(path.clone().into_path()); //FIXME: remove clone
    println!("Build: {:?}", &ctx.meta.file_path);
    ctx.meta.file_name = Some(path.path().file_stem().unwrap().to_str().unwrap().to_string());
    ctx.meta.title = ctx.meta.file_name.clone();
    match fs::read_to_string(path.clone().into_path()) {
        Ok(source) => ctx.parse(&source),
        Err(_) => return Err("can't read the file"),
    };
    let html = match panic::catch_unwind(|| ctx.to_html()) {
        Ok(s) => {
            if cfg!(windows) {
                s.replace('\n', "\r\n")
            }
            else {
                s
            }
        }
        Err(_) => return Err("can not build this file"),
    };
    let mut out = out.clone();
    out.push(&ctx.meta.file_name.unwrap());
    out.set_extension("md");
    let mut file = File::create(out).expect("can not create file");
    file.write_all(html.as_bytes()).expect("can not write to file");
    Ok(())
}
