use std::fmt::{Display, Formatter};
use crate::Command;
use std::fmt;

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let a: Vec<String> = self.args.iter().map(|v| format!("{}", v)).collect();
        let kv: Vec<String> = self.kvs.iter().map(|(k, v)| format!("{} = {}", k, v)).collect();

        write!(f, "\\{}{{{}}}", self.cmd, [&a[..], &kv[..]].concat().join(", "))
    }
}