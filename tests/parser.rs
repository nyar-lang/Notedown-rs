extern crate notedown;
extern crate pest;

use notedown::utils::{parse, token_print};
use notedown::{NotedownAST as AST, NotedownParser, NotedownRule, ToHTML};
use pest::Parser;

#[test]
fn text_chinese_english() {
    const TEXT: &str = "text\nabc一二三0123\nabc123\n";
    token_print(TEXT, NotedownRule::program);
    let result = parse(TEXT);
    assert_eq!(
        format!("{:?}", result),
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("abc"), Word("一二三"), Word("0123"), Newline, Word("abc123")]))])"#
    )
}

#[test]
fn text_style_1() {
    const TEXT: &str = "text\n**";
    token_print(TEXT, NotedownRule::program);
    let result = parse(TEXT);
    assert_eq!(format!("{}", result), r#"unknown"#);
    assert_eq!(
        format!("{:?}", result),
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("**")]))])"#
    );
    assert_eq!(result.to_html_default(), "<p>text\n**</p>");
}

#[test]
fn text_style_2() {
    const TEXT: &str = "text\n***";
    token_print(TEXT, NotedownRule::program);
    let result = parse(TEXT);
    assert_eq!(format!("{}", result), r#"unknown"#);
    assert_eq!(
        format!("{:?}", result),
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("***")]))])"#
    );
    assert_eq!(result.to_html_default(), "<p>text\n***</p>");
}

#[test]
fn text_style_3() {
    const TEXT: &str = "text\n* * *";
    token_print(TEXT, NotedownRule::program);
    let result = parse(TEXT);
    assert_eq!(format!("{}", result), r#"unknown"#);
    assert_eq!(
        format!("{:?}", result),
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Italic(String(" "), 1), Word("*")]))])"#
    );
    assert_eq!(result.to_html_default(), "<p>text\n<i> </i>*</p>");
}

#[test]
fn text_style_4() {
    const TEXT: &str = "text\n** \\** **";
    token_print(TEXT, NotedownRule::program);
    assert_eq!(
        format!("{:?}", parse(TEXT)),
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Bold(Statements([Escaped("*"), Word("*")]), 2)]))])"#
    )
}

#[test]
fn text_style_5() {
    const TEXT: &str = "text\n*** * ***";
    token_print(TEXT, NotedownRule::program);
    assert_eq!(
        format!("{:?}", parse(TEXT)),
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Bold(Italic(Statements([Word("*")]), 0), 3)]))])"#
    )
}

#[test]
fn text_style_6() {
    const TEXT: &str = "text\n**** * ****";
    token_print(TEXT, NotedownRule::program);
    assert_eq!(
        format!("{:?}", parse(TEXT)),
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("**** * ****")]))])"#
    )
}

#[test]
fn header() {
    const TEXT: &str = "## Header1";
    token_print(TEXT, NotedownRule::program);
    assert_eq!(
        format!("{:?}", parse(TEXT)),
        r#"Statements([Header(Statements([Word("Header1")]), 2, {})])"#
    )
}
