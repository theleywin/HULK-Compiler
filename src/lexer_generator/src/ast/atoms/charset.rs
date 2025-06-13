use std::fmt::Display;

/// Represents a set of character ranges, with optional negation.
///
/// `CharSet` is typically used in lexical analyzers and regular expression engines
/// to define character classes, such as `[a-z]`, `[0-9]`, or `[^a-z]`.
///
/// # Fields
/// - `range`: A vector of `(char, char)` tuples representing inclusive character ranges.
///            For instance, `[('a', 'z'), ('0', '9')]` matches all lowercase letters and digits.
/// - `neg`: Indicates whether the character set is negated (e.g., `[^...]` in regex).
///
/// # Behavior
/// - Ranges are normalized on creation: `(start, end)` pairs are sorted, and reversed if needed.
/// - Implements `PartialEq<char>` to check character membership (or exclusion if negated).
/// - Implements `Display` to generate a string like `[a-z]` or `[^A-Z]`.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharSet {
    pub range: Vec<(char, char)>,
    pub neg: bool,
}

impl CharSet {
    /// Constructs a new `CharSet`, normalizing and sorting the provided ranges.
    pub fn new(range: Vec<(char, char)>, neg : bool) -> Self {
        let mut range = range;
        for range in &mut range {
            if range.0 > range.1 {
                std::mem::swap(&mut range.0, &mut range.1);
            }
        }
        range.sort();
        CharSet { range, neg }
    }
}

impl PartialEq<char> for CharSet {
    /// Checks whether a given character matches the character set.
    /// Takes into account the negation flag.
    fn eq(&self, other: &char) -> bool {
        self.neg
            ^ self
                .range
                .iter()
                .any(|&(start, end)| *other >= start && *other <= end)
    }
}

impl Display for CharSet {
    /// Returns a string representation of the character set in regex format.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = String::new();
        repr.push('[');
        if self.neg {
            repr.push('^');
        }
        for &(start, end) in &self.range {
            repr.push(start);
            repr.push('-');
            repr.push(end);
        }
        repr.push(']');
        write!(f, "{}", repr)
    }
}