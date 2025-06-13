use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Index,
};

/// A tracker that manages seen and unseen items, 
/// providing indexing and efficient membership checks.
///
/// `VisitTracker` maintains a set of seen items, a list of unseen items 
/// to be processed, and assigns a unique index to each seen item.
///
/// # Type parameters
///
/// * `T` - The type of elements tracked. Must implement `Eq`, `Hash`, and `Clone`.
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
    /// Creates a new, empty `VisitTracker`.
    pub fn new() -> Self {
        Self {
            unseen: Vec::new(),
            seen: HashSet::new(),
            index: HashMap::new(),
        }
    }

    /// Adds an item to the unseen list if it has not been seen before.
    ///
    /// Returns `true` if the item was added, or `false` if it was already seen.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to add.
    ///
    pub fn add_unseen(&mut self, item: T) -> bool {
        if self.seen.contains(&item) {
            return false;
        }
        self.seen.insert(item.clone());
        self.index.insert(item.clone(), self.index.len());
        self.unseen.push(item);
        true
    }

    /// Pops an item from the unseen list.
    ///
    /// Returns `Some(item)` if there is an unseen item to process, or `None` if empty.
    ///
    /// Note: This pops from the end of the unseen list (LIFO order).
    ///
    pub fn pop_unseen(&mut self) -> Option<T> {
        self.unseen.pop()
    }

    /// Checks if an item has been seen.
    ///
    /// Returns `true` if the item was added before, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to check.
    ///
    pub fn is_seen(&self, item: &T) -> bool {
        self.seen.contains(item)
    }

    /// Returns an iterator over all seen items.
    ///
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

    /// Returns an iterator over the seen items.
    ///
    /// This enables `for item in &tracker` syntax.
    fn into_iter(self) -> Self::IntoIter {
        self.seen.iter()
    }
}

impl<T> Index<&T> for VisitTracker<T>
where
    T: Eq + Hash + Clone,
{
    type Output = usize;

    /// Returns the unique index associated with the given item.
    ///
    /// # Panics
    ///
    /// Panics if the item is not found in the tracker.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to index.
    ///
    fn index(&self, item: &T) -> &Self::Output {
        &self.index[item]
    }
}