use notedown_fmt::Settings;
use std::{fs::File, io::prelude::*};

#[test]
fn test_math() {
    let cfg = Settings::default();
    let mut file = File::create("tests/text/conda.out.note").unwrap();
    let f = cfg.format(include_str!("conda.note"));
    // println!("{:#?}", f);
    file.write_all(f.as_bytes()).unwrap();
}
