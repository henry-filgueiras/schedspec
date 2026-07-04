//! Small support types shared across the kernel.

use serde::{Deserialize, Serialize};

/// A vector that provably holds at least one element.
///
/// Used wherever the docs make evidence mandatory: a `ScopedDisagreement`
/// without residue or a revocation without supporting records is
/// unrepresentable because this type has no empty constructor.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NonEmpty<T> {
    head: T,
    tail: Vec<T>,
}

impl<T> NonEmpty<T> {
    pub fn new(head: T) -> Self {
        Self {
            head,
            tail: Vec::new(),
        }
    }

    pub fn from_vec(mut items: Vec<T>) -> Option<Self> {
        if items.is_empty() {
            return None;
        }
        let head = items.remove(0);
        Some(Self { head, tail: items })
    }

    pub fn push(&mut self, item: T) {
        self.tail.push(item);
    }

    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn first(&self) -> &T {
        &self.head
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.head).chain(self.tail.iter())
    }
}

impl<T> From<NonEmpty<T>> for Vec<T> {
    fn from(ne: NonEmpty<T>) -> Vec<T> {
        let mut v = vec![ne.head];
        v.extend(ne.tail);
        v
    }
}
