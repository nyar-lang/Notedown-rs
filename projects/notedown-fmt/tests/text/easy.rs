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

#[test]
fn test_pangu() {
    let s = Settings::default();
    const INPUT: &str = "新八的構造成分有95%是眼鏡、3%是水、2%是垃圾";
    const OUTPUT: &str = "新八的構造成分有 95% 是眼鏡、3% 是水、2% 是垃圾\n";
    assert_eq!(s.format(&s.format(INPUT)), OUTPUT)
}
