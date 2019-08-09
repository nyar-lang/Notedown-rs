extern crate notedown;
extern crate pest;

use notedown::utils::{parse, token_print};
use notedown::{NotedownParser, NotedownRule, ToAST};
use pest::Parser;

#[test]
fn main() {
    const TEXT: &str = "\
# Header1
Line1

## Header2
Line2
Line3
### Header
Line4
";
    token_print(TEXT, NotedownRule::program);
}

#[test]
fn header() {
    const TEXT: &str = "## Header1";
    parse(TEXT);
   // panic!()
}
