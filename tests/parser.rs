extern crate notedown;
extern crate pest;

use notedown::utils::token_print;
use notedown::{traits::ToAST, NotedownParser, NotedownRule};
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
