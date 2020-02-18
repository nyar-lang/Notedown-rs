mod io;
mod music;
mod value;

use crate::{Context, AST};
pub use io::import;
pub use music::meting_js;
use std::collections::HashMap;
pub use value::Value;

impl Context {
    pub fn execute_cmd(&self, ast: AST) -> AST {
        match &ast {
            AST::Command(cmd, args, kvs) => {
                let out = match cmd.as_str() {
                    "netease" => meting_js("netease", args, kvs),
                    "tencent_music" => meting_js("tencent", args, kvs),
                    "kugou" => meting_js("kugou", args, kvs),
                    "xiami" => meting_js("xiami", args, kvs),
                    "baidu_music" => meting_js("baidu", args, kvs),
                    _ => None,
                };
                match out {
                    None => AST::Raw(format!("{}", ast)),
                    Some(s) => AST::String(s),
                }
            }
            _ => AST::None,
        }
    }
}
