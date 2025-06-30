/// Regex module
/// 
/// This module contains regex parsing, matching, and AST functionality.

pub mod matcher;
pub mod ast;
pub mod parser;

// Re-export for convenience
pub use matcher::{DFAMatcher, NFAMatcher};
pub use parser::RegexParser;
