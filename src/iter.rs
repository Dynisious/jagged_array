//! Author --- DMorgan  
//! Last Moddified --- 2021-03-13

use crate::Array;
use alloc::alloc::Allocator;
use core::{
  fmt,
  slice::Iter as SIter,
  iter::{Copied, Iterator, IntoIterator, FusedIterator,},
};

/// An iterator over the ranks (row) of an [`Array`](crate::Array).
pub struct Iter<'a, T,> {
  /// The position within `elements`.
  start_pos: usize,
  /// The rank dimensions.
  dimensions: Copied<SIter<'a, usize>>,
  /// The elements being referenced.
  elements: &'a [T],
}

impl<'a, T,> Iterator for Iter<'a, T,> {
  type Item = &'a [T];

  #[inline]
  fn size_hint(&self,) -> (usize, Option<usize>,) { self.dimensions.size_hint() }
  fn next(&mut self,) -> Option<Self::Item> {
    let end = self.start_pos + self.dimensions.next()?;
    let rank = &self.elements[self.start_pos..end];

    self.start_pos = end; Some(rank)
  }
}

impl<'a, T,> FusedIterator for Iter<'a, T,> {}

impl<T,> Clone for Iter<'_, T,> {
  fn clone(&self,) -> Self {
    Iter {
      start_pos: self.start_pos,
      dimensions: self.dimensions.clone(),
      elements: self.elements,
    }
  }
}

impl<T,> fmt::Debug for Iter<'_, T,>
  where T: fmt::Debug, {
  fn fmt(&self, fmt: &mut fmt::Formatter,) -> fmt::Result {
    struct Helper<'a, T,>(&'a Iter<'a, T,>,);

    impl<T,> fmt::Debug for Helper<'_, T,>
      where T: fmt::Debug, {
      fn fmt(&self, fmt: &mut fmt::Formatter,) -> fmt::Result {
        fmt.debug_list().entries(self.0.clone(),).finish()
      }
    }

    fmt.debug_tuple(stringify!(Iter),).field(&Helper(self,),).finish()
  }
}

impl<T, A,> Array<T, A,>
  where A: Allocator, {
  /// Returns an iterator over all of the ranks (rows) of this `Array`.
  pub fn iter<'a,>(&'a self,) -> Iter<'a, T,> {
    Iter {
      start_pos: 0,
      dimensions: self.dimensions.iter().copied(),
      elements: &self.elements,
    }
  }
}

impl<'a, T, A,> IntoIterator for &'a Array<T, A,>
  where A: Allocator, {
  type IntoIter = Iter<'a, T,>;
  type Item = &'a [T];

  #[inline]
  fn into_iter(self,) -> Self::IntoIter { self.iter() }
}

#[cfg(test,)]
mod tests {
  use super::*;
  use alloc::vec::Vec;

  #[test]
  fn test_iter() {
    let array = crate::array![[1, 2]; 1, 2, 3];
    let elements = array.iter().collect::<Vec<_>>();
    assert_eq!(elements, alloc::vec![alloc::vec![1], alloc::vec![2, 3]],);
  }
}
