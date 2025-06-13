use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Index,
};

pub struct VisitTracker<T>
where
    T: Eq + Hash + Clone,
{
    unseen: Vec<T>,
    seen: HashSet<T>,
    index: HashMap<T, usize>,
}

impl<T> VisitTracker<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            unseen: Vec::new(),
            seen: HashSet::new(),
            index: HashMap::new(),
        }
    }

    pub fn add_unseen(&mut self, item: T) -> bool {
        if self.seen.contains(&item) {
            return false;
        }
        self.seen.insert(item.clone());
        self.index.insert(item.clone(), self.index.len());
        self.unseen.push(item);
        true
    }

    pub fn pop_unseen(&mut self) -> Option<T> {
        self.unseen.pop()
    }

    pub fn is_seen(&self, item: &T) -> bool {
        self.seen.contains(item)
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<T> {
        self.seen.iter()
    }
}

impl<'a, T> IntoIterator for &'a VisitTracker<T>
where
    T: Eq + Hash + Clone,
{
    type Item = &'a T;
    type IntoIter = std::collections::hash_set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.seen.iter()
    }
}

impl<T> Index<&T> for VisitTracker<T>
where
    T: Eq + Hash + Clone,
{
    type Output = usize;

    fn index(&self, item: &T) -> &Self::Output {
        &self.index[item]
    }
}