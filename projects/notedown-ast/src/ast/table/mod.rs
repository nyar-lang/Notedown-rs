use crate::AST;

mod from;

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct TableView {
    head: Vec<AST>,
    align: Vec<u8>,
    terms: Vec<Vec<AST>>,
    column: usize,
}

impl Display for TableView {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}
