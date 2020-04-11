mod display;
mod from;

use crate::{Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Command {
    cmd: Box<str>,
    args: Vec<Value>,
    kvs: HashMap<String, Value>
}

