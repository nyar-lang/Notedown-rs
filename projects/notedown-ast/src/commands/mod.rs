mod io;
mod value;
mod music;

use std::collections::HashMap;
pub use value::Value;
pub use music::meting_js;
pub use io::import;
use crate::{Context, AST};


impl Context {
    fn execute_cmd(&self, ast: AST) -> AST {
        match ast {
            AST::Command(cmd, args, kvs) => {
                let out = match cmd.as_str() {
                    "netease" => meting_js("netease", args, kvs),
                    _ => None
                };
                match out {
                    None => AST::Raw(format!("{}", ast)),
                    Some(s) => AST::String(s)
                }
            }
            _ => AST::None
        }
    }
}