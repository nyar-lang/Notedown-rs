extern crate notedown;
extern crate pest;

use notedown::utils::{parse, token_print};
use notedown::{NotedownAST as AST, NotedownParser, NotedownRule, ToAST};
use pest::Parser;

#[test]
fn text_chinese_english() {
    const TEXT: &str = "text\nabc一二三0123\nabc123\n";
    token_print(TEXT, NotedownRule::program);
    assert_eq!(
        format!("{:?}",parse(TEXT)),
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("abc"), Word("一二三"), Word("0123"), Newline, Word("abc123")]))])"#
    )
}

#[test]
fn text_style_1() {
    const TEXT: &str = "text\n**";
    token_print(TEXT, NotedownRule::program);
    assert_eq!(
        format!("{:?}", parse(TEXT)),
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("**")]))])"#
    )
}

#[test]
fn text_style_2() {
    const TEXT: &str = "text\n***";
    token_print(TEXT, NotedownRule::program);
    assert_eq!(
        format!("{:?}", parse(TEXT)),
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("***")]))])"#
    )
}

#[test]
fn text_style_3() {
    const TEXT: &str = "text\n* * *";
    token_print(TEXT, NotedownRule::program);
    assert_eq!(
        format!("{:?}", parse(TEXT)),
        r#"Statements([Paragraph(Statements([Word("text"), Newline, None, Word("*")]))])"#
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
