/// Core lexer functionality module
/// 
/// This module contains the main lexer implementation and related types.

pub mod lexer;
pub mod rule;
pub mod token;
pub mod result;

// Re-export main types for convenience
pub use lexer::Lexer;
pub use rule::Rule;
pub use token::{LexerChunk, Token};
pub use result::LexerResult;
