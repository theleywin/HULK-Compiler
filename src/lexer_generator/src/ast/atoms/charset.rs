use std::fmt::Display;

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
            if start == end {
                repr.push_str(&escape_char(start));
            } else {
                repr.push_str(&escape_char(start));
                repr.push('-');
                repr.push_str(&escape_char(end));
            }
        }
        repr.push(']');
        write!(f, "{}", repr)
    }
}

fn escape_char(c: char) -> String {
    match c {
        '\\' => String::from("\\\\"),
        '-' => String::from("\\-"),
        ']' => String::from("\\]"),
        '^' => String::from("\\^"),
        '\n' => String::from("\\n"),
        '\r' => String::from("\\r"),
        '\t' => String::from("\\t"),
        c if c.is_control() || (!c.is_ascii() && !c.is_alphanumeric()) => {
            format!("\\u{{{:X}}}", c as u32)
        }
        _ => c.to_string(),
    }
}