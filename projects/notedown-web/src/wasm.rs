use notedown_ast::{Context, NotedownTarget, NotedownTemplate, Settings, ToHTML, AST};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rs_render(input: &str) -> String {
    let mut c = Context {
        ast: AST::None,
        meta: Default::default(),
        cfg: Settings { tab_size: 2, template: NotedownTemplate::Vue, target: NotedownTarget::Web },
    };
    c.parse(input);
    c.to_html()
}

#[test]
fn test() {
    println!("{}", rs_render("## 2"));
}
