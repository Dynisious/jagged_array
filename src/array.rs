//! Author --- DMorgan  
//! Last Moddified --- 2021-02-13

use alloc::{
  vec::Vec,
  alloc::{
    Allocator,
    Global,
  },
};
use core::{
  fmt,
  ops::{Index, IndexMut,},
  borrow::Borrow,
  iter::{Extend, FromIterator,},
};

#[doc(hidden)]
#[inline]
pub fn from_parts<T,>(dimensions: Vec<usize>, elements: Vec<T>,) -> Array<T,> {
  let expected = dimensions.iter().copied().sum::<usize>();
  assert!(expected == elements.len(), "wrong number of elements; expected `{}`, found `{}`", expected, elements.len(),);

  Array { dimensions, elements, }
}

#[doc(hidden)]
#[inline]
pub fn from_elem<T,>(element: T, dimensions: Vec<usize>,) -> Array<T,>
  where T: Clone, {
  let len = dimensions.iter().copied().sum();
  from_parts(dimensions, alloc::vec![element; len],)
}

/// A dynamically sized jagged array type.
/// 
/// An `Array` can be indexed either by a `rank/file` pair or simply by a `rank`.
#[derive(Eq, Clone,)]
pub struct Array<T, A: Allocator = Global,> {
  /// The dimensions of the `Array`.
  pub(crate) dimensions: Vec<usize, A>,
  /// The elements of the `Array`.
  pub(crate) elements: Vec<T, A>,
}

impl<T,> Array<T, Global,> {
  /// Creates a new empty `Array` with no dimensions or elements.
  #[inline]
  pub const fn new() -> Self {
    Self {
      dimensions: Vec::new(),
      elements: Vec::new(),
    }
  }
  /// Creates a new empty `Array` with no dimensions and space for at least `capacity`
  /// elements.
  /// 
  /// # Params
  /// 
  /// capacity --- The number of elements to create space for.  
  pub fn with_capacity(capacity: usize,) -> Self {
    Self {
      dimensions: Vec::new(),
      elements: Vec::with_capacity(capacity,),
    }
  }
}

impl<T, A,> Array<T, A,>
  where A: Allocator + Clone, {
  /// Creates a new empty `Array` with no dimensions or elements.
  /// 
  /// # Params
  /// 
  /// alloc --- The allocator to use.  
  pub fn new_in(alloc: A,) -> Self {
    Self {
      dimensions: Vec::new_in(alloc.clone(),),
      elements: Vec::new_in(alloc,),
    }
  }
  /// Creates a new empty `Array` with no dimensions and space for at least `capacity`
  /// elements.
  /// 
  /// # Params
  /// 
  /// capacity --- The number of elements to create space for.  
  /// alloc --- The allocator to use.  
  pub fn with_capacity_in(capacity: usize, alloc: A,) -> Self {
    Self {
      dimensions: Vec::new_in(alloc.clone(),),
      elements: Vec::with_capacity_in(capacity, alloc,),
    }
  }
}

impl<T, A,> Array<T, A,>
  where A: Allocator, {
  /// Returns a reference to the underlying allocator.
  #[inline]
  pub fn allocator(&self,) -> &A { self.elements.allocator() }
  /// Returns the number of elements in the `Array`.
  #[inline]
  pub fn len(&self,) -> usize { self.elements.len() }
  /// Returns the number of elements there is space for.
  #[inline]
  pub fn capacity(&self,) -> usize { self.elements.capacity() }
  /// Returns the number of ranks (rows) making up the `Array`.
  #[inline]
  pub fn ranks(&self,) -> usize { self.dimensions.len() }
  /// Reserves space for at least `additional` more elements.
  #[inline]
  pub fn reserve(&mut self, additional: usize,) { self.elements.reserve(additional,) }
  /// Reserves space for exactly `additional` more elements.
  #[inline]
  pub fn reserve_exact(&mut self, additional: usize,) { self.elements.reserve(additional,) }
  /// Returns the number of files (columns) within the given `rank`.
  /// 
  /// Defaults to `0` for nonexistant ranks.
  /// 
  /// # Params
  /// 
  /// rank --- The rank to get the files for.  
  pub fn files(&self, rank: usize,) -> usize { self.dimensions.get(rank,).copied().unwrap_or(0,) }
  /// Returns the index of the element positioned at `index`.
  /// 
  /// # Params
  /// 
  /// index --- The rank and file of the element being indexed.  
  pub(crate) unsafe fn get_element_index_unchecked(&self, index: &[usize; 2],) -> usize {
    self.dimensions.iter().take(index[0],).sum::<usize>() + index[1]
  }
  /// Returns the index of the element positioned at `index`.
  /// 
  /// # Params
  /// 
  /// index --- The rank and file of the element being indexed.  
  pub(crate) fn get_element_index(&self, index: &[usize; 2],) -> Option<usize> {
    if self.dimensions.len() <= index[0] || self.dimensions[index[0]] <= index[1] { return None }

    Some(unsafe { self.get_element_index_unchecked(index,) })
  }
  /// A panic free way to index an `Array`.
  /// 
  /// # Params
  /// 
  /// index --- The rank and file of the element.  
  pub fn get(&self, index: impl Borrow<[usize; 2]>,) -> Option<&T> {
    self.get_element_index(index.borrow(),).map(|index,| &self.elements[index],)
  }
  /// A panic free way to index an `Array`.
  /// 
  /// # Params
  /// 
  /// index --- The rank and file of the element.  
  pub fn get_mut(&mut self, index: impl Borrow<[usize; 2]>,) -> Option<&mut T> {
    self.get_element_index(index.borrow(),).map(move |index,| &mut self.elements[index],)
  }
  /// A panic free way to index a rank of an `Array`.
  /// 
  /// # Params
  /// 
  /// rank --- The rank to reference.  
  pub fn rank(&self, rank: usize,) -> Option<&[T]> {
    if rank >= self.ranks() { return None }

    let start = self.dimensions.iter().copied().take(rank,).sum();
    let end = start + self.dimensions[rank];

    Some(&self.elements[start..end])
  }
  /// Inserts an element into the rank of the inner `Array`.
  /// 
  /// # Panics
  /// 
  /// Panics if `rank` is outside the range `[0, ranks]`.
  /// 
  /// # Params
  /// 
  /// rank --- The index within the ranks to insert the values.  
  /// elements --- The elements of the rank.  
  pub fn insert(&mut self, rank: usize, elements: Vec<T, impl Allocator>,) {
    assert!(rank <= self.ranks(), "`rank` was greater than `ranks`",);

    self.dimensions.insert(rank, elements.len(),);
    //Safe because we just inserted the new rank into `dimensions`.
    let start = unsafe { self.get_element_index_unchecked(&[rank, 0],) };
    self.elements.splice(start..start, elements,);
  }
  /// Removes and returns an element from the rank of the inner `Array`.
  /// 
  /// # Panics
  /// 
  /// Panics if `rank` is ouside the range `[0, ranks)`.
  /// 
  /// # Params
  /// 
  /// rank --- The index of the rank to remove.  
  pub fn remove(&mut self, rank: usize,) -> Vec<T> {
    assert!(rank < self.ranks(), "`rank` was greter than `ranks`",);

    //Safe because we just asserted that rank is within the bounds of `ranks`.
    let start = unsafe { self.get_element_index_unchecked(&[rank, 0],) };
    let end = start + self.dimensions.remove(rank,);
    self.elements.drain(start..end,).collect()
  }
  /// Appends a rank to the inner `Array`.
  /// 
  /// # Params
  /// 
  /// rank --- The elements to append.
  pub fn push(&mut self, rank: Vec<T, impl Allocator>,) {
    self.dimensions.push(rank.len(),);
    self.elements.extend(rank,);
  }
  /// Pops a rank from the inner `Array`.
  pub fn pop(&mut self,) -> Option<Vec<T>> {
    let files = self.dimensions.pop()?;
    let start = self.elements.len() - files;

    Some(self.elements.drain(start..).collect())
  }
}

impl<T, A,> Default for Array<T, A,>
  where A: Allocator + Clone + Default, {
  fn default() -> Self { Self::new_in(A::default(),) }
}

impl<T, U, A,> PartialEq<Array<U, A,>> for Array<T, A,>
  where T: PartialEq<U>,
    A: Allocator, {
  fn eq(&self, rhs: &Array<U, A,>,) -> bool {
    self.dimensions == rhs.dimensions && self.elements == rhs.elements
  }
}

impl<T, A,> Index<usize> for Array<T, A,>
  where A: Allocator, {
  type Output = [T];

  fn index(&self, index: usize,) -> &Self::Output {
    self.rank(index,).expect("`index` is not within the `Array` bounds")
  }
}

impl<T, A,> IndexMut<usize> for Array<T, A,>
  where A: Allocator, {
  fn index_mut(&mut self, index: usize,) -> &mut Self::Output {
    self.rank_mut(index,).expect("`index` is not within the `Array` bounds").into()
  }
}

impl<T, A,> Index<[usize; 2]> for Array<T, A,>
  where A: Allocator, {
  type Output = T;

  fn index(&self, index: [usize; 2],) -> &Self::Output {
    self.get(index,).expect("`index` is not within the `Array` bounds")
  }
}

impl<T, A,> IndexMut<[usize; 2]> for Array<T, A,>
  where A: Allocator, {
  fn index_mut(&mut self, index: [usize; 2],) -> &mut Self::Output {
    self.get_mut(index,).expect("`index` is not within the `Array` bounds")
  }
}

impl<T, A,> FromIterator<A> for Array<T, Global,>
  where Self: Extend<A>, {
  fn from_iter<I,>(iter: I,) -> Self
    where I: IntoIterator<Item = A>, {
    let iter = iter.into_iter();
    let mut array = Self::with_capacity(iter.size_hint().0,);

    array.extend(iter,); array
  }
}

impl<'a, T: 'a, A,> Extend<&'a [T]> for Array<T, A,>
  where T: Copy,
    A: Allocator, {
  fn extend<I,>(&mut self, ranks: I,)
    where I: IntoIterator<Item = &'a [T]>, {
    let ranks = ranks.into_iter();
    self.dimensions.reserve(ranks.size_hint().0,);

    for rank in ranks {
      self.dimensions.push(rank.len(),);
      self.elements.extend(rank,);
    }
  }
}

impl<'a, T: 'a, A,> Extend<&'a mut [T]> for Array<T, A,>
  where A: Allocator,
    Self: Extend<&'a [T]>, {
  fn extend<I,>(&mut self, ranks: I,)
    where I: IntoIterator<Item = &'a mut [T]>, {
    self.extend(ranks.into_iter().map(|x,| &*x,))
  }
}

impl<T, A1, A2,> Extend<Vec<T, A2>> for Array<T, A1,>
  where A1: Allocator,
    A2: Allocator, {
  fn extend<I,>(&mut self, ranks: I,)
    where I: IntoIterator<Item = Vec<T, A2>>, {
    let ranks = ranks.into_iter();
    self.dimensions.reserve(ranks.size_hint().0,);

    for rank in ranks { self.push(rank,) }
  }
}

impl<T, A,> fmt::Debug for Array<T, A,>
  where T: fmt::Debug,
    A: Allocator, {
  fn fmt(&self, fmt: &mut fmt::Formatter,) -> fmt::Result {
    fmt.debug_list()
    .entries(self.iter(),)
    .finish()
  }
}

#[cfg(test,)]
mod tests {
  #[test]
  fn test_array() {
    let mut array = crate::array![[2, 3]; 1, 2, 3, 4, 5];
    assert_eq!(array.capacity(), 5);
    assert_eq!(array.len(), 5);
    assert_eq!(array.ranks(), 2);
    assert_eq!(array.files(0), 2);
    assert_eq!(array.files(1), 3);
    assert_eq!(array.get([1, 1]), Some(&4));
    assert_eq!(array.get_mut([1, 1]), Some(&mut 4));
    assert_eq!(array.rank(1), Some(&[3, 4, 5][..]));

    array.insert(1, alloc::vec![6, 7, 8]);
    assert_eq!(array, crate::array![[2, 3, 3]; 1, 2, 6, 7, 8, 3, 4, 5]);
    assert_eq!(array.remove(0), [1, 2][..]);
    assert_eq!(array.pop(), Some(alloc::vec![3, 4, 5]));
    array.pop();
    assert_eq!(array.pop(), None);

    array.push(alloc::vec![3, 4, 5]);
    assert_eq!(array, crate::array![[3]; 3, 4, 5]);

    array.extend(alloc::vec![alloc::vec![6, 7], alloc::vec![8, 9, 10]]);
    assert_eq!(array, crate::array![[3, 2, 3]; 3, 4, 5, 6, 7, 8, 9, 10]);
  }
}
