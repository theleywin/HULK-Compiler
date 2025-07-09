use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Token<T>
where
    T: Eq + Hash,
{
    pub ty: T,
    pub slice: String,
    pub start: usize,
    pub end: usize,
}

impl<T> Token<T>
where
    T: Eq + Hash,
{
    pub fn new(ty: T, slice: String, start: usize, end: usize) -> Self {
        Self {
            ty,
            slice,
            start,
            end,
        }
    }
}
