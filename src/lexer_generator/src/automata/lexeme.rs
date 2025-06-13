use std::hash::Hash;

pub struct Lexeme<'a, T>
where
    T: Eq + PartialEq + Clone + Hash,
{
    pub kind: T,
    pub fragment: &'a str,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl<'a, T> Lexeme<'a, T>
where
    T: Eq + PartialEq + Clone + Hash,
{
    pub fn with(kind: T, fragment: &'a str, line: usize, start: usize, end: usize) -> Self {
        Self {
            kind,
            fragment,
            line,
            start,
            end,
        }
    }

    pub fn is_blank(&self) -> bool {
        self.start == self.end
    }
}