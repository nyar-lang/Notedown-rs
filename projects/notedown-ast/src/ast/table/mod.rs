use crate::AST;

mod from;

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct TableView {
    pub head: Vec<AST>,
    pub align: Vec<u8>,
    pub terms: Vec<Vec<AST>>,
    pub column: usize,
}

impl Display for TableView {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}
