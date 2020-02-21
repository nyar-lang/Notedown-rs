mod io;
mod music;
mod value;

use crate::{Context, AST};
pub use io::{import, set_categories, set_date, set_series, set_tags, set_title};
pub use music::meting_js;
pub use value::Value;

impl Context {
    pub fn execute_cmd(&mut self, ast: AST) -> AST {
        match &ast {
            AST::Command(cmd, args, kvs) => {
                let out = match cmd.as_str() {
                    "comment" => Some(String::new()),
                    "title" => set_title(self, args),
                    "date" => set_date(self, args),
                    "tags" => set_tags(self, args),
                    "categories" => set_categories(self, args),
                    "series" => set_series(self, args),
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
