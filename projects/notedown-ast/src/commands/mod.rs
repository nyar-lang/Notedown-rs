mod io;
mod media;
mod value;

use crate::{
    commands::{
        io::set_file_name,
        media::{fancy_quote, image_insert, link_insert},
    },
    Context, MissingCommand, AST, GLOBAL_CONFIG,
};
#[cfg(feature = "desktop")]
use carbon_lib::{utils::CarbonHTML, CarbonError, Config};
pub use io::{import, set_categories, set_date, set_series, set_tags, set_title};
pub use media::meting_js;
use std::collections::{HashMap, VecDeque};
pub use value::Value;

impl Context {
    pub fn execute_cmd(&mut self, ast: AST) -> AST {
        match &ast {
            AST::Command(cmd, args, kvs) => {
                macro_rules! args {
                    () => {
                        VecDeque::from(args.clone())
                    };
                }
                macro_rules! kvs {
                    () => {
                        kvs.clone()
                    };
                }
                let cmd = cmd.as_str().to_lowercase();
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

                    "quote" => return fancy_quote(self, args, kvs!()),
                    "import" => import(self, args!(), kvs!()),
                    "img" | "image" => image_insert(self, args!(), kvs!()),
                    "link" => link_insert(args, kvs!()),

                    "netease" => meting_js("netease", args, kvs),
                    "kugou" => meting_js("kugou", args, kvs),
                    "xiami" => meting_js("xiami", args, kvs),
                    "baidu_music" => meting_js("baidu", args, kvs),
                    "tencent_music" => meting_js("tencent", args, kvs),
                    _ => try_render(cmd, args!(), kvs!()),
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
            AST::Command(cmd, args, kvs) => match cmd.as_str().to_lowercase().as_str() {
                "toc" => String::new(),
                _ => match cfg.template {
                    MissingCommand::Vue => format!("<{0}>{1:?}{2:?}</{0}>", cmd, kvs, args),
                    MissingCommand::Zola => format!("{{% {0} %}}", cmd),
                },
            },
            _ => String::new(),
        }
    }
}

#[cfg(feature = "desktop")]
fn try_render(cmd: String, mut args: VecDeque<Value>, mut kvs: HashMap<String, Value>) -> Option<String> {
    let body = match kvs.remove("body") {
        Some(s) => s.trim().to_string(),
        None => return None,
    };
    let mut title: Option<String> = None;
    match kvs.remove("title") {
        Some(s) => title = Some(s.to_string()),
        None => {
            if let Some(s) = args.pop_front() {
                title = Some(s.to_string())
            }
        }
    };
    let mut carbon = Config::default();
    carbon.html_type = CarbonHTML::Raw;
    carbon.syntax = cmd;
    carbon.file_title = title;
    match carbon.render_html(&body) {
        Err(_) => return None,
        Ok(o) => Some(o),
    }
}

#[cfg(not(feature = "desktop"))]
fn try_render(cmd: String, mut args: VecDeque<Value>, mut kvs: HashMap<String, Value>) -> Option<String> {
    Some(format!("\\{}{:?}{:?}", cmd, args, kvs))
}
