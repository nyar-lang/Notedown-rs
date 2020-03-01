use notedown_ast::{Context, MissingCommand, NotedownMeta, NotedownTarget, ToHTML, AST};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rs_render(input: &str) -> String {
    let mut c = Context::default();
    c.parse(input);
    c.to_html()
}

#[test]
fn test() {
    println!("{}", rs_render("## 2"));
}
