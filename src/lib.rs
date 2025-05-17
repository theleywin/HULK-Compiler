
pub mod ast_nodes;
pub mod visitor;
pub mod semantic_analyzer;
pub mod tokens;

lalrpop_util::lalrpop_mod!(pub parser);

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::ast_nodes::expression::Expression;
//     use crate::ast_nodes::program::Statement;
// }