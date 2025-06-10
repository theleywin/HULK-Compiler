use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharSet {
    pub range: Vec<(char, char)>,
    pub neg: bool,
}

impl CharSet {
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
    fn eq(&self, other: &char) -> bool {
        self.neg
            ^ self
                .range
                .iter()
                .any(|&(start, end)| *other >= start && *other <= end)
    }
}

impl Display for CharSet {
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