#[macro_use]
extern crate notedown;

use notedown::{dict, NotedownAST as AST, ToHTML};
use std::collections::HashMap;

#[test]
fn bold_italic() {
    let s = AST::from("bold_italic");
    let b = AST::Bold(Box::new(s), 2);
    let i = AST::Italic(Box::new(b), 1);
    let fmt = i.to_html_default();
    assert_eq!(fmt, "<i><b>bold_italic</b></i>")
}

#[test]
fn italic_bold() {
    let s = AST::from("bold_italic");
    let i = AST::Italic(Box::new(s), 2);
    let b = AST::Bold(Box::new(i), 1);
    let fmt = b.to_html_default();
    assert_eq!(fmt, "<b><i>bold_italic</i></b>")
}

#[test]
fn font() {
    let mut dict = dict! {
        "face"=>"黑体",
        "size"=>"20"
    };
    let s = AST::from("黑体20");
    let f = AST::Font(Box::new(s), dict);
    let fmt = f.to_html_default();
    assert_eq!(fmt, "<font face=\"黑体\" size=\"20\">黑体20</font>")
}
