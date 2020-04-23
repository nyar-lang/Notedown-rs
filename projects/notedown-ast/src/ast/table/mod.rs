use crate::AST;

mod from;

use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, Clone)]
pub struct TableView<'a> {
    head: Vec<AST<'a>>,
    align: Vec<u8>,
    terms: Vec<Vec<AST<'a>>>,
    column: usize,
}


impl<'a> Display for TableView<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}
