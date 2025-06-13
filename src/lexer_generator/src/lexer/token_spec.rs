use std::hash::Hash;

pub struct TokenSpec<T>
where
    T: Eq + PartialEq + Clone + Hash,
{
    pub patt: String,
    pub kind: T,
    pub ignore: bool,
}

impl<T> TokenSpec<T>
where
    T: Eq + PartialEq + Clone + Hash,
{
    pub fn build(kind: T, patt: impl Into<String>) -> Self {
        Self {
            patt: patt.into(),
            kind,
            ignore: false,
        }
    }

    pub fn build_ignorable(kind: T, patt: impl Into<String>) -> Self {
        Self {
            patt: patt.into(),
            kind,
            ignore: true,
        }
    }
}