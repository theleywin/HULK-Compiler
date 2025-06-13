use std::hash::Hash;

/// Defines a token specification consisting of a pattern, token kind, and
/// whether the token should be ignored (e.g., whitespace).
///
/// # Type Parameters
///
/// * `T`: The token kind type, which must implement `Eq`, `PartialEq`, `Clone`, and `Hash`.
pub struct TokenSpec<T>
where
    T: Eq + PartialEq + Clone + Hash,
{
    /// The pattern string, typically a regex, that defines the token.
    pub patt: String,

    /// The kind or category of the token.
    pub kind: T,

    /// Whether tokens matching this pattern should be ignored by the lexer.
    pub ignore: bool,
}

impl<T> TokenSpec<T>
where
    T: Eq + PartialEq + Clone + Hash,
{
    /// Creates a new token specification with the given kind and pattern.
    /// The token will not be ignored by default.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token kind.
    /// * `patt` - The pattern string (e.g., regex) describing the token.
    ///
    /// # Returns
    ///
    /// A `TokenSpec` instance with `ignore` set to `false`.
    pub fn build(kind: T, patt: impl Into<String>) -> Self {
        Self {
            patt: patt.into(),
            kind,
            ignore: false,
        }
    }

    /// Creates a new token specification with the given kind and pattern,
    /// marking the token as ignorable by the lexer (e.g., whitespace).
    ///
    /// # Arguments
    ///
    /// * `kind` - The token kind.
    /// * `patt` - The pattern string describing the token.
    ///
    /// # Returns
    ///
    /// A `TokenSpec` instance with `ignore` set to `true`.
    pub fn build_ignorable(kind: T, patt: impl Into<String>) -> Self {
        Self {
            patt: patt.into(),
            kind,
            ignore: true,
        }
    }
}