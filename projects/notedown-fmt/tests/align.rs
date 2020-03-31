use notedown_fmt::Settings;

#[test]
fn test_text_1() {
    let s = Settings::default();
    const INPUT: &str = "
    4
   3
  2
 1
0
";
    const OUTPUT: &str = "\n    4\n    3\n    2\n    1\n    0\n";
    assert_eq!(s.format(&s.format(INPUT)), OUTPUT)
}

#[test]
fn test_text_2() {
    let s = Settings::default();
    const INPUT: &str = "
0
 1
  2
 1
0
";
    const OUTPUT: &str = "\n0\n 1\n  2\n 1\n0\n";
    assert_eq!(s.format(&s.format(INPUT)), OUTPUT)
}

#[test]
fn test_text_3() {
    let s = Settings::default();
    const INPUT: &str = "
0
 1
  2
   3
    4
";
    const OUTPUT: &str = "\n0\n 1\n  2\n   3\n    4\n";
    assert_eq!(s.format(&s.format(INPUT)), OUTPUT)
}

#[test]
fn test_quote_1() {
    let s = Settings::default();
    const INPUT: &str = "
    >4
   3
  >2
 1
0
";
    const OUTPUT: &str = "\n";
    //const OUTPUT: &str = "\n    >  4\n    >  3\n    >  2\n    >  1\n    >  0\n";
    assert_eq!(s.format(&s.format(INPUT)), OUTPUT)
}

#[test]
fn test_quote_2() {
    let s = Settings::default();
    const INPUT: &str = "
    4
   >3
  2
 >1
0
";
    const OUTPUT: &str = "\n    4\n    >3\n    2\n    >1\n    0\n";
    assert_eq!(s.format(&s.format(INPUT)), OUTPUT)
}

#[test]
fn test_quote_3() {
    let s = char::default();
    println!("{:?}", s)
}
