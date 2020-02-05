#[macro_use]
extern crate notedown;
extern crate pest;

use notedown::utils::{parse, token_print};
use notedown::{NotedownAST as AST, NotedownParser, NotedownRule, ToHTML};
use pest::Parser;

#[test]
fn text_chinese_english() {
    const TEXT: &str = "text\nabc一二三0123\n\tabc123\n";
    token_print(TEXT, NotedownRule::program);
    string_eq!(
        "text\nabc 一二三 0123\nabc123\n\n",
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("abc"), Word("一二三"), Word("0123"), Newline, Word("abc123")]))])"#,
        "<p>text\nabc 一二三 0123\nabc123</p>"
    );
}

#[test]
fn text_style_1() {
    const TEXT: &str = "text\n**";
    token_print(TEXT, NotedownRule::program);
    string_eq!(
        "text\n**\n\n",
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("**")]))])"#,
        "<p>text\n**</p>"
    );
}

#[test]
fn text_style_2() {
    const TEXT: &str = "text\n***";
    token_print(TEXT, NotedownRule::program);
    string_eq!(
        "text\n***\n\n",
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("***")]))])"#,
        "<p>text\n***</p>"
    );
}

#[test]
fn text_style_3() {
    const TEXT: &str = "text\n* * *";
    token_print(TEXT, NotedownRule::program);
    string_eq!(
        "text\nUnimplementedError*\n\n",
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Italic(String(" "), 1), Word("*")]))])"#,
        "<p>text\n<i> </i>*</p>"
    );
}

#[test]
fn text_style_4() {
    const TEXT: &str = "text\n** \\** **";
    token_print(TEXT, NotedownRule::program);
    string_eq!(
        "text\nUnimplementedError\n\n",
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Bold(Statements([Escaped("*"), Word("*")]), 2)]))])"#,
        "<p>text\n<b>Escaped(\"*\")*</b></p>"
    );
}

#[test]
fn text_style_5() {
    const TEXT: &str = "text\n*** * ***";
    token_print(TEXT, NotedownRule::program);
    string_eq!(
        "text\nUnimplementedError\n\n",
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Bold(Italic(Statements([Word("*")]), 0), 3)]))])"#,
        "<p>text\n<b><i>*</i></b></p>"
    );
}

#[test]
fn text_style_6() {
    const TEXT: &str = "text\n**** * ****";
    token_print(TEXT, NotedownRule::program);
    string_eq!(
        "text\n**** * ****\n\n",
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("**** * ****")]))])"#,
        "<p>text\n**** * ****</p>"
    );
}

#[test]
fn text_math_1() {
    const TEXT: &str = "text\n$ x $";
    token_print(TEXT, NotedownRule::program);
    string_eq!(
        "text\n$x$\n\n",
        r#"Statements([Paragraph(Statements([Word("text"), Newline, MathInline("x")]))])"#,
        "<p>text\n$x$</p>"
    );
}

#[test]
fn text_math_2() {
    const TEXT: &str = "text\n$$ \\frac{x}{2} $$";
    token_print(TEXT, NotedownRule::program);
    string_eq!(
       "text\n$\\displaystyle\\frac{x}{2}$\n\n",
        r#"Statements([Paragraph(Statements([Word("text"), Newline, MathInline("\\displaystyle\\frac{x}{2}")]))])"#,
        "<p>text\n$\\displaystyle\\frac{x}{2}$</p>"
    );
}

#[test]
fn text_math_3() {
    const TEXT: &str = "text\n$$$ \\frac{x}{2} $$$";
    token_print(TEXT, NotedownRule::program);
    string_eq!(
        "text\n$$$ \\frac{x}{2} $$$\n\n",
        r#"Statements([Paragraph(Statements([Word("text"), Newline, Word("$$$ \\frac{x}{2} $$$")]))])"#,
        "<p>text\n$$$ \\frac{x}{2} $$$</p>" //FIXME: use <span> to protect $$$
    );
}
