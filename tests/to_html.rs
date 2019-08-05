extern crate notedown;

use notedown::{HTMLConfig, AST as AST, ToHTML};

#[test]
fn bold_italic() {
    let s = AST::String("bold_italic".to_string());
    let b = AST::Bold(Box::new(s));
    let i = AST::Italic(Box::new(b));
    let fmt = i.to_html(HTMLConfig::default());
    assert_eq!(fmt, "<i><b>bold_italic</b></i>")
}

#[test]
fn italic_bold() {
    let s = AST::String("bold_italic".to_string());
    let i = AST::Italic(Box::new(s));
    let b = AST::Bold(Box::new(i));
    let fmt = b.to_html(HTMLConfig::default());
    assert_eq!(fmt, "<b><i>bold_italic</i></b>")
}
