//! Author --- DMorgan  
//! Last Moddified --- 2021-03-13

use crate::Array;
use alloc::alloc::{Allocator, Global,};
use core::{
  fmt,
  ops::{Deref, DerefMut,},
  convert::{AsRef, AsMut,},
  borrow::{Borrow, BorrowMut,},
  slice::IterMut,
  iter::{Extend, IntoIterator,},
};

/// A mutable reference to a rank in an [`Array`](crate::Array).
#[derive(Eq,)]
pub struct Rank<'a, T, A: Allocator = Global,> {
  rank: usize,
  start_pos: usize,
  array: &'a mut Array<T, A,>
}

impl<T, A,> Rank<'_, T, A,>
  where A: Allocator, {
  /// Returns a reference to the inner slice.
  pub fn as_slice(&self,) -> &[T] {
    let end = self.start_pos + self.array.files(self.rank,);

    &self.array.elements[self.start_pos..end]
  }
  /// Returns a mutable reference to the inner slice.
  pub fn as_mut_slice(&mut self,) -> &mut [T] {
    let end = self.start_pos + self.array.files(self.rank,);

    &mut self.array.elements[self.start_pos..end]
  }
  /// Inserts an element into the rank of the inner `Array`.
  /// 
  /// # Complexity
  /// 
  /// O(N) where `N is Array.len`.
  /// 
  /// # Panics
  /// 
  /// Panics if `index` is outside the range `[0, len]`.
  /// 
  /// # Params
  /// 
  /// index --- The index within the rank to insert the value.  
  /// value --- The value to insert into the rank.  
  pub fn insert(&mut self, index: usize, value: T,) {
    let files = &mut self.array.dimensions[self.rank];
    assert!(index <= *files, "`index` was greater than `len`",);

    *files += 1;
    self.array.elements.insert(self.start_pos + index, value,);
  }
  /// Removes and returns an element from the rank of the inner `Array`.
  /// 
  /// # Complexity
  /// 
  /// O(N) where `N is Array.len`.
  /// 
  /// # Panics
  /// 
  /// Panics if `index` is ouside the range `[0, len)`.
  /// 
  /// # Params
  /// 
  /// index --- The index of the element to remove.  
  pub fn remove(&mut self, index: usize,) -> T {
    let files = &mut self.array.dimensions[self.rank];
    assert!(index <= *files, "`index` was greter than `len`",);

    *files -= 1;
    self.array.elements.remove(self.start_pos + index,)
  }
  /// Appends an element to the rank of the inner `Array`.
  /// 
  /// # Complexity
  /// 
  /// O(N) where `N is Array.len`.
  /// 
  /// # Params
  /// 
  /// value --- The element to append.
  pub fn push(&mut self, value: T,) {
    let files = &mut self.array.dimensions[self.rank];
    self.array.elements.insert(self.start_pos + *files, value,);
    *files += 1;
  }
  /// Pops an element from the rank of the inner `Array`.
  /// 
  /// # Complexity
  /// 
  /// O(N) where `N is Array.len`.
  pub fn pop(&mut self,) -> Option<T> {
    let files = &mut self.array.dimensions[self.rank];

    *files = files.checked_sub(1,)?;
    Some(self.array.elements.remove(self.start_pos + *files,))
  }
}

impl<T, A,> PartialEq for Rank<'_, T, A,>
  where T: PartialEq,
    A: Allocator, {
  fn eq(&self, rhs: &Self,) -> bool { self == rhs.as_slice() }
}

impl<T, U, A,> PartialEq<[U]> for Rank<'_, T, A,>
  where T: PartialEq<U>,
    A: Allocator, {
  fn eq(&self, rhs: &[U],) -> bool { self.as_slice() == rhs }
}

impl<T, A,> Extend<T> for Rank<'_, T, A,>
  where A: Allocator, {
  fn extend<I,>(&mut self, iter: I,)
    where I: IntoIterator<Item = T>, {
    let iter = iter.into_iter();
    self.array.reserve(iter.size_hint().0,);

    let files = &mut self.array.dimensions[self.rank];
    let end = self.start_pos + *files;
    let mut len = 0;
    self.array.elements.splice(end..end, iter.inspect(|_,| len += 1,),);
    *files += len;
  }
}

impl<'a, T: 'a, A,> Extend<&'a T> for Rank<'_, T, A,>
  where T: Copy,
    A: Allocator, {
  fn extend<I,>(&mut self, iter: I,)
    where I: IntoIterator<Item = &'a T>, {
    self.extend(iter.into_iter().copied(),)
  }
}

impl<T, A,> AsRef<[T]> for Rank<'_, T, A,>
  where A: Allocator, {
  #[inline]
  fn as_ref(&self,) -> &[T] { self.as_slice() }
}

impl<T, A,> AsMut<[T]> for Rank<'_, T, A,>
  where A: Allocator, {
  #[inline]
  fn as_mut(&mut self,) -> &mut [T] { self.as_mut_slice() }
}

impl<T, A,> Borrow<[T]> for Rank<'_, T, A,>
  where A: Allocator, {
  #[inline]
  fn borrow(&self,) -> &[T] { self.as_slice() }
}

impl<T, A,> BorrowMut<[T]> for Rank<'_, T, A,>
  where A: Allocator, {
  #[inline]
  fn borrow_mut(&mut self,) -> &mut [T] { self.as_mut_slice() }
}

impl<T, A,> Deref for Rank<'_, T, A,>
  where A: Allocator, {
  type Target = [T];

  #[inline]
  fn deref(&self,) -> &Self::Target { self.as_slice() }
}

impl<T, A,> DerefMut for Rank<'_, T, A,>
  where A: Allocator, {
  #[inline]
  fn deref_mut(&mut self,) -> &mut Self::Target { self.as_mut_slice() }
}

impl<'a, T, A,> From<Rank<'a, T, A,>> for &'a mut [T]
  where A: Allocator, {
  #[inline]
  fn from(mut from: Rank<'a, T, A,>,) -> Self { unsafe { &mut *(from.as_mut_slice() as *mut [T]) } }
}

impl<T, A,> fmt::Debug for Rank<'_, T, A,>
  where T: fmt::Debug,
    A: Allocator, {
  fn fmt(&self, fmt: &mut fmt::Formatter,) -> fmt::Result {
    fmt.debug_tuple(stringify!(Rank),).field(&&**self,).finish()
  }
}

impl<'a, T, A,> IntoIterator for Rank<'a, T, A,>
  where A: Allocator, {
  type IntoIter = IterMut<'a, T,>;
  type Item = &'a mut T;

  fn into_iter(self,) -> Self::IntoIter { <&mut [T]>::from(self,).into_iter() }
}

impl<T, A,> Array<T, A,>
  where A: Allocator, {
  /// A panic free way to index a rank of an `Array`.
  /// 
  /// # Params
  /// 
  /// rank --- The rank to reference.  
  pub fn rank_mut<'a,>(&'a mut self, rank: usize,) -> Option<Rank<'a, T, A,>> {
    Some(Rank {
      rank,
      start_pos: self.get_element_index(&[rank, 0,],)?,
      array: self,
    })
  }
}

#[cfg(test,)]
mod tests {
  #[test]
  fn test_rank() {
    let mut array = crate::array![[1, 2]; 1, 2, 3];
    let mut rank = array.rank_mut(0,).expect("failed to get the rank");
    assert_eq!(rank, [1][..],);

    rank.insert(1, 4);
    assert_eq!(rank, [1, 4][..]);
    rank.push(5);
    assert_eq!(rank, [1, 4, 5][..]);
    assert_eq!(array, crate::array![[3, 2]; 1, 4, 5, 2, 3]);

    let mut rank = array.rank_mut(1,).expect("failed to get the rank");
    assert_eq!(rank, [2, 3][..]);
    assert_eq!(rank.remove(1), 3);
    assert_eq!(rank.pop(), Some(2));
    assert_eq!(rank.pop(), None);

    rank.extend([6, 7].iter());
    assert_eq!(rank, [6, 7][..]);
    assert_eq!(array, crate::array![[3, 2]; 1, 4, 5, 6, 7]);
  }
}
