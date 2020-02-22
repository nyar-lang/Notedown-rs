use notedown_fmt::Settings;


#[test]
fn test_text() {
    let s = Settings::default();
    const INPUT:&str =        r#"
    4
   3
  2
 1
0
"#;
    const OUTPUT:&str = r#"
    4
    3
    2
    1
    0
"#;
    assert_eq!(s.format(&s.format(INPUT)), OUTPUT)
}

#[test]
fn test_quote() {
    let s = Settings::default();
    const INPUT:&str =        r#"
    >4
   >3
  >2
 >1
>0
"#;
    const OUTPUT:&str = r#"
    4
    3
    2
    1
    0
"#;
    assert_eq!(s.format(&s.format(INPUT)), OUTPUT)
}
