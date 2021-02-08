use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct Arena<T> {
    inner: Vec<T>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Arena { inner: vec![] }
    }
}

impl<T> Arena<T> {
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.inner.iter_mut()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Index<T> {
    index: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    _pd: std::marker::PhantomData<*const T>,
}

impl<T> Clone for Index<T> {
    fn clone(&self) -> Index<T> {
        Index {
            index: self.index,
            _pd: std::marker::PhantomData,
        }
    }
}

impl<T> PartialEq for Index<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T> fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index({})", self.index)
    }
}

impl<T> Copy for Index<T> {}

impl<T> std::ops::Index<Index<T>> for Arena<T> {
    type Output = T;
    fn index(&self, index: Index<T>) -> &Self::Output {
        &self.inner[index.index]
    }
}

impl<T> std::ops::IndexMut<Index<T>> for Arena<T> {
    fn index_mut(&mut self, index: Index<T>) -> &mut Self::Output {
        &mut self.inner[index.index]
    }
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena { inner: vec![] }
    }

    pub fn push(&mut self, val: T) -> Index<T> {
        let out = Index {
            index: self.inner.len(),
            _pd: std::marker::PhantomData,
        };
        self.inner.push(val);
        out
    }
}
