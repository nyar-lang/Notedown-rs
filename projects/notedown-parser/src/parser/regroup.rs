use notedown_ast::AST;
use pest::iterators::Pair;
use crate::note_down::Rule;

pub fn regroup_list_view(lists: &[(usize, &str, Vec<AST>)]) -> Vec<AST> {
    vec![]
}

pub fn regroup_table_view(table: &[Vec<Vec<AST>>]) -> Vec<AST> {
    for line in table {
        for item in line {
            println!("{:?}", item)
        }
    }
    vec![]
}
