use notedown_fmt::Settings;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rs_format(input: &str, tab_size: usize) -> String {
    let c = Settings { tab_size };
    c.format(input)
}
