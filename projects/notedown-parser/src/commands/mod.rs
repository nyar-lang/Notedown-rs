mod desktop;
mod media;
mod value;

use crate::{Context, NotedownBackend, AST};
pub use desktop::{import, set_categories, set_date, set_file_name, set_series, set_tags, set_title, try_render_code};
pub use media::{fancy_quote, image_insert, link_insert, meting_js};
pub use value::Value;

impl Context {
    pub fn execute_cmd(&mut self, ast: AST) -> AST {
        match &ast {
            AST::Command(cmd, args, kvs) => {
                let cmd = cmd.to_lowercase();
                let out = match cmd.as_str() {
                    "comment" => Some(String::new()),
                    "toc" => return ast,
                    "more" | "read_more" => return AST::from("\n\n<!-- more -->\n\n"),
                    "title" => set_title(self, args),
                    "date" => set_date(self, args),
                    "name" | "path" => set_file_name(self, args),
                    "tags" => set_tags(self, args),
                    "categories" | "cats" => set_categories(self, args),
                    "series" => set_series(self, args),

                    "quote" => return fancy_quote(self, args, kvs),
                    "import" => import(self, args, kvs),
                    "img" | "image" => image_insert(self, args, kvs),
                    "link" => link_insert(self, args, kvs),

                    "netease" => meting_js("netease", args, kvs),
                    "kugou" => meting_js("kugou", args, kvs),
                    "xiami" => meting_js("xiami", args, kvs),
                    "baidu_music" => meting_js("baidu", args, kvs),
                    "tencent_music" => meting_js("tencent", args, kvs),
                    _ => try_render_code(cmd, args, kvs),
                };
                match out {
                    None => AST::Raw(format!("{}", ast)),
                    Some(s) => AST::String(s),
                }
            }
            _ => AST::None,
        }
    }
    pub fn execute_cmd_missing(&mut self, ast: AST) -> String {
        let ref cfg = GLOBAL_CONFIG.lock().unwrap();
        match &ast {
            AST::Command(cmd, args, kvs) => match cmd.to_lowercase().as_str() {
                "toc" => String::new(),
                _ => match cfg.target {
                    NotedownBackend::Vue => format!("<{0}>{1:?}{2:?}</{0}>", cmd, kvs, args),
                    NotedownBackend::Zola => format!("{{% {0} %}}", cmd),
                    NotedownBackend::Web => String::new(),
                    NotedownBackend::VSCode => String::new(),
                    NotedownBackend::Yew => String::new(),
                },
            },
            _ => String::new(),
        }
    }
}
