use notedown_ast::{parse, ToHTML};
use notedown_fmt::Settings;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rs_format(input: &str) -> String {
    let c = Settings::default();
    c.format(input)
}

#[wasm_bindgen]
pub fn rs_render(input: &str) -> String {
    let c = parse(input);
    c.to_html()
}
