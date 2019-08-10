#[macro_use]
extern crate notedown;
extern crate pest;

use notedown::utils::{parse, token_print};
use notedown::{NotedownAST as AST, NotedownParser, NotedownRule, ToHTML};
use pest::Parser;

#[test]
fn header() {
    const TEXT: &str = "## Header1";
    token_print(TEXT, NotedownRule::program);
    string_eq!(
        "unknown",
        r#"Statements([Header(Statements([Word("Header1")]), 2, {})])"#,
        "Header1 2{}"
    );
}
