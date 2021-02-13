//! Author --- DMorgan  
//! Last Moddified --- 2021-03-13

use crate::Array;
use alloc::alloc::Allocator;
use core::{
  fmt,
  slice::Iter as SIter,
  iter::{Copied, Iterator, IntoIterator, FusedIterator,},
};

/// An iterator over the ranks (rows) of an [`Array`](crate::Array).
pub struct IterMut<'a, T: 'a,> {
  /// The position within `elements`.
  start_pos: usize,
  /// The rank dimensions.
  dimensions: Copied<SIter<'a, usize>>,
  /// The elements being referenced.
  elements: *mut T,
}

impl<'a, T,> Iterator for IterMut<'a, T,> {
  type Item = &'a mut [T];

  #[inline]
  fn size_hint(&self,) -> (usize, Option<usize>,) { self.dimensions.size_hint() }
  fn next(&mut self,) -> Option<Self::Item> {
    let len = self.dimensions.next()?;
    let rank = unsafe { core::slice::from_raw_parts_mut(self.elements.add(self.start_pos,), len,) };

    self.start_pos += len; Some(rank)
  }
}

impl<'a, T,> FusedIterator for IterMut<'a, T,> {}

impl<T,> Clone for IterMut<'_, T,> {
  fn clone(&self,) -> Self {
    IterMut {
      start_pos: self.start_pos,
      dimensions: self.dimensions.clone(),
      elements: self.elements,
    }
  }
}

impl<T,> fmt::Debug for IterMut<'_, T,>
  where T: fmt::Debug, {
  fn fmt(&self, fmt: &mut fmt::Formatter,) -> fmt::Result {
    struct Helper<'a, T,>(&'a IterMut<'a, T,>,);

    impl<'a, T: 'a,> fmt::Debug for Helper<'a, T,>
      where T: fmt::Debug, {
      fn fmt(&self, fmt: &mut fmt::Formatter,) -> fmt::Result {
        let iter = IterMut {
          start_pos: self.0.start_pos,
          dimensions: self.0.dimensions.clone(),
          elements: self.0.elements,
        };

        fmt.debug_list().entries(iter,).finish()
      }
    }

    fmt.debug_tuple(stringify!(Iter),).field(&Helper(self,),).finish()
  }
}

impl<T, A,> Array<T, A,>
  where A: Allocator, {
  /// Returns a mutable iterator over all of the ranks (rows) of this `Array`.
  pub fn iter_mut<'a,>(&'a mut self,) -> IterMut<'a, T,> {
    IterMut {
      start_pos: 0,
      dimensions: self.dimensions.iter().copied(),
      elements: self.elements.as_mut_ptr(),
    }
  }
}

impl<'a, T, A,> IntoIterator for &'a mut Array<T, A,>
  where A: Allocator, {
  type IntoIter = IterMut<'a, T,>;
  type Item = &'a mut [T];

  #[inline]
  fn into_iter(self,) -> Self::IntoIter { self.iter_mut() }
}

#[cfg(test,)]
mod tests {
  use super::*;
  use alloc::vec::Vec;

  #[test]
  fn test_iter_mut() {
    let mut array = crate::array![[1, 2]; 1, 2, 3];
    let elements = array.iter_mut().collect::<Vec<_>>();
    assert_eq!(elements, alloc::vec![alloc::vec![1], alloc::vec![2, 3]],);
  }
}
