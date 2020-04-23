mod from;

use std::fmt::{Display, Formatter};
use std::fmt;
use crate::{Value};
use std::collections::HashMap;
use crate::lazy_format;

#[derive(Debug, Clone)]
pub struct Command<'a> {
    pub(crate) cmd: &'static str,
    pub(crate) args: Vec<Value<'a>>,
    pub(crate) kvs: HashMap<&'static str, Value<'a>>,
    pub(crate) kind: CommandKind,
}

#[derive(Copy, Clone, Debug)]
pub enum CommandKind {
    ///
    /// ```md
    /// \cmd: args
    /// ```
    Inline,
    /// ```md
    /// \cmd(
    ///     arg = 1
    /// )
    /// ```
    Normal,

    /// `[]`
    SmartLink,
    /// ````md
    /// ```cmd(arg=1)
    /// body text
    /// ```
    /// ````
    CodeBlock,
    /// ```md
    /// <cmd arg=1>body text</cmd>
    /// ```
    OpenClose,
    /// ```md
    /// `<cmd arg=1/>`
    /// ```
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
