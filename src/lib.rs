//! A growable array that expands to provide a mutable reference to items beyond the stored
//! collection.
//!
//! The [`ExpandVec::expand_get_mut`] method allows for mutably indexing into single items of the
//! collection and always getting a valid reference to an item. If necessary, the inner collection
//! is expanded to precisely fit the index only if that index is beyond the length of the
//! collection. A mutable reference to that new item is returned. When the index does fit within the
//! existing collection, a mutable reference to that item is simply returned, without altering the
//! inner collection.
use std::slice::SliceIndex;

/// A growable array that expands to provide a mutable reference to items beyond the stored
/// collection.
///
/// The inner collection is a regular [`Vec`].
#[derive(Debug, Clone)]
pub struct ExpandVec<T: Default + Clone> {
    inner: Vec<T>,
}

impl<T: Default + Clone> ExpandVec<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    pub fn push(&mut self, value: T) {
        self.inner.push(value)
    }

    /// Returns a reference to an element or subslice depending on the type of index.
    ///
    /// * If given a position, returns a reference to the element at that position or `None` if out
    ///   of bounds.
    /// * If given a range, returns the subslice corresponding to that range, or `None`
    ///   if out of bounds.
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[T]>,
    {
        self.inner.get(index)
    }

    /// Returns a mutable reference to an element or subslice depending on the type of index
    /// (see [`get`]) or `None` if the index is out of bounds.
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[T]>,
    {
        self.inner.get_mut(index)
    }

    /// Always returns a mutable reference to an element.
    /// If the index points beyond the contents of the inner collection, it is expanded with
    /// default values to fit the index. In that case, a mutable reference to this last item under
    /// the index is returned.
    pub fn expand_get_mut(&mut self, index: usize) -> &mut T {
        if index > self.inner.len() {
            let remaining = index - self.inner.len();
            self.inner.extend(vec![Default::default(); remaining])
        }
        // We can safely unwrap since the inner Vec was extended by a sufficient number of items.
        self.inner.get_mut(index).unwrap()
    }

    /// Returns the inner [`Vec`].
    pub fn raw_vec(self) -> Vec<T> {
        self.inner
    }
}

impl<T: Default + Clone> Default for ExpandVec<T> {
    fn default() -> Self {
        Self::new()
    }
}
