extern crate notedown;
extern crate pest;

use notedown::utils::token_print;
use notedown::{NotedownParser, NotedownRule};
use pest::Parser;

const TEXT: &str = r#"

# Header1
Line1

## Header2
Line2
### Header
Line3
"#;

#[test]
fn main() {
    token_print(TEXT);
    panic!()
}
