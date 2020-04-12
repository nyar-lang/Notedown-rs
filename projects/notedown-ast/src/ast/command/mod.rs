mod from;
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::{Value};
use std::collections::HashMap;
use crate::lazy_format;

#[derive(Debug, Clone)]
pub struct Command<'a> {
    cmd: &'static str,
    args: Vec<Value<'a>>,
    kvs: HashMap<&'static str, Value<'a>>,
    kind: CommandKind,
}

#[derive(Copy, Clone, Debug)]
pub enum CommandKind {
    Inline,
    Smart,
    Normal,
    OpenClose,
    SelfClose,
}

impl<'a> Display for Command<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let a: Vec<String> = self.args.iter().map(|v| format!("{}", v)).collect();
        let kv: Vec<String> = self.kvs.iter().map(|(k, v)| format!("{} = {}", k, v)).collect();

        write!(f, "\\{}{{{}}}", self.cmd, [&a[..], &kv[..]].concat().join(", "))
    }
}

pub fn html_tag<'a>(tag: &'a str, content: impl Display + 'a) -> impl Display + 'a {
    lazy_format!("<{tag}>{content}</{tag}>", tag=tag, content=content)
}
