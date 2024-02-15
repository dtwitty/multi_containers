use std::collections::BTreeSet;
use crate::sets::Set;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TreeSet<T: Ord> {
    data: BTreeSet<T>,
}

impl<T: Ord> TreeSet<T> {
    pub fn new() -> Self {
        TreeSet {
            data: BTreeSet::new(),
        }
    }
}

impl<'a, T: Ord + 'a> Set<'a> for TreeSet<T> {
    type Elem = T;
    type Iter = impl Iterator<Item=&'a T> + 'a;

    fn insert(&mut self, value: Self::Elem) -> bool {
        self.data.insert(value)
    }

    fn remove(&mut self, value: &Self::Elem) -> bool {
        self.data.remove(value)
    }

    fn contains(&self, value: &Self::Elem) -> bool {
        self.data.contains(value)
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn iter(&'a self) -> Self::Iter {
        self.data.iter()
    }
}
