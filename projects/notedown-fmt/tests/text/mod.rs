use notedown_fmt::Settings;
mod easy;

#[test]
fn test_pangu() {
    let s = Settings::default();
    const INPUT: &str = "新八的構造成分有95%是眼鏡、3%是水、2%是垃圾";
    const OUTPUT: &str = "新八的構造成分有 95% 是眼鏡、3% 是水、2% 是垃圾\n";
    assert_eq!(s.format(&s.format(INPUT)), OUTPUT)
}
