use std::hash::Hash;

/// Represents a lexeme (token) recognized by a lexer, containing
/// information about the token's kind, the matched text fragment, and its position.
///
/// # Type Parameters
///
/// * `T`: The type used to represent the kind/category of the lexeme (e.g., token type).
///        Must implement `Eq`, `PartialEq`, `Clone`, and `Hash`.
pub struct Lexeme<'a, T>
where
    T: Eq + PartialEq + Clone + Hash,
{
    /// The token kind or category.
    pub kind: T,

    /// The string slice corresponding to the matched fragment.
    pub fragment: &'a str,

    /// The line number where the lexeme was found (1-based).
    pub line: usize,

    /// The start index (byte offset) of the lexeme in the source text.
    pub start: usize,

    /// The end index (byte offset) of the lexeme in the source text.
    pub end: usize,
}

impl<'a, T> Lexeme<'a, T>
where
    T: Eq + PartialEq + Clone + Hash,
{
    /// Creates a new `Lexeme` with the specified kind, fragment, line, and start/end positions.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token kind or category.
    /// * `fragment` - The matched substring slice.
    /// * `line` - The line number where the lexeme was found.
    /// * `start` - The starting byte index of the lexeme.
    /// * `end` - The ending byte index of the lexeme.
    ///
    /// # Returns
    ///
    /// A new instance of `Lexeme`.
    pub fn with(kind: T, fragment: &'a str, line: usize, start: usize, end: usize) -> Self {
        Self {
            kind,
            fragment,
            line,
            start,
            end,
        }
    }

    /// Checks if the lexeme is blank, meaning it has zero length.
    ///
    /// # Returns
    ///
    /// `true` if the start and end indices are equal, indicating an empty lexeme.
    /// Otherwise, returns `false`.
    pub fn is_blank(&self) -> bool {
        self.start == self.end
    }
}