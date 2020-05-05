use markdown::tokenize;
use notedown_ast::AST;
use std::{fs::File, io::Write};

const HEADS: &str = "\
# Title 1
## Title 2
### Title 3
#### Title 4
##### Title 5
###### Title 6
####### Title 7

# Title **bold**
";

#[test]
pub fn headers() {
    let raw = tokenize(HEADS);
    let out = AST::from(raw.clone());
    println!("{:#?}", raw);
    println!("{:#?}", out);
}

#[test]
pub fn markdown_codegen() {
    let out = AST::from(tokenize(include_str!("readme.md")));
    let mut file = File::create("tests/markdown/readme.note").unwrap();
    file.write_all(format!("{}", out).as_bytes()).unwrap();
}
