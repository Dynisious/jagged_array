//! Author --- DMorgan  
//! Last Moddified --- 2021-03-13

use crate::Array;
use alloc::{
  vec::{Vec, IntoIter as VIter,},
  alloc::Allocator,
};
use core::iter::{
  Iterator,
  Enumerate, Scan, Flatten,
  IntoIterator,
  FusedIterator,
  TrustedLen,
  SourceIter,
  InPlaceIterable,
};

/// An iterator over the elements of an [`Array`](crate::Array).
pub struct IntoIter<T, A,>(Flatten<Scan<Enumerate<VIter<usize, A>>, VIter<T, A>, fn(&mut VIter<T, A>, (usize, usize)) -> Option<Vec<([usize; 2], T)>>>>,)
  where A: Allocator,;

impl<T, A,> Iterator for IntoIter<T, A,>
  where A: Allocator, {
  type Item = ([usize; 2], T,);

  #[inline]
  fn size_hint(&self,) -> (usize, Option<usize>,) { self.0.size_hint() }
  #[inline]
  fn next(&mut self,) -> Option<Self::Item> { self.0.next() }
}

unsafe impl<T, A,> SourceIter for IntoIter<T, A,>
  where A: Allocator, {
  type Source = Self;

  #[inline]
  unsafe fn as_inner(&mut self,) -> &mut Self::Source { self }
}

impl<T, A,> FusedIterator for IntoIter<T, A,>
  where A: Allocator, {}

unsafe impl<T, A,> TrustedLen for IntoIter<T, A,>
  where A: Allocator, {}

unsafe impl<T, A,> InPlaceIterable for IntoIter<T, A,>
  where A: Allocator, {}

impl<T, A,> IntoIterator for Array<T, A,>
  where A: Allocator, {
  type IntoIter = IntoIter<T, A,>;
  type Item = ([usize; 2], T,);

  fn into_iter(self,) -> Self::IntoIter {
    fn map_rank<T, A,>(elems: &mut VIter<T, A>, (rank, files,): (usize, usize,),) -> Option<Vec<([usize; 2], T,)>>
      where A: Allocator, {
      Some(elems.take(files,).enumerate().map(move |(file, x,),| ([rank, file,], x,),).collect::<Vec<_>>())
    }

    let map_rank: fn(&mut VIter<T, A>, (usize, usize)) -> Option<Vec<([usize; 2], T)>> = map_rank;
    IntoIter(self.dimensions.into_iter().enumerate().scan(self.elements.into_iter(), map_rank,).flatten(),)
  }
}

#[cfg(test,)]
mod tests {
  use super::*;

  #[test]
  fn test_into_iter() {
    let array = crate::array![[1, 2]; 1, 2, 3];
    let elements = array.into_iter().collect::<Vec<_>>();
    assert_eq!(elements, alloc::vec![([0, 0], 1), ([1, 0], 2), ([1, 1], 3)],);
  }
}
