extern crate notedown;

use notedown::{HTMLConfig, NotedownAST as AST, ToHTML};
use std::collections::HashMap;

macro_rules! dict (
    {$($key:expr => $value:expr),+} => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $value.to_string());
            )+
            map
        }
     };
);

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

#[test]
fn font() {
    let mut dict = dict! {
        "face"=>"黑体",
        "size"=>"20"
    };
    let s = AST::String("黑体20".to_string());
    let f = AST::Font(Box::new(s), dict);
    let fmt = f.to_html_default();
    assert_eq!(fmt, "<font size=\"20\" face=\"黑体\">黑体20</font>")
}
