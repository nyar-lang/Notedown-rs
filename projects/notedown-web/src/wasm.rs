use notedown_ast::{Context, MissingCommand, NotedownMeta, NotedownTarget, ToHTML, AST};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rs_render(input: &str) -> String {
    let mut c = Context {
        ast: AST::None,
        meta: Default::default(),
        cfg: NotedownMeta { tab_size: 2, template: MissingCommand::Vue, target: NotedownTarget::Web },
    };
    c.parse(input);
    c.to_html()
}

#[test]
fn test() {
    println!("{}", rs_render("## 2"));
}
