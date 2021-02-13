//! A library which provides a `Vec`-like structure which is equivilant to a
//! `Vec<Vec<T>>` type but with less allocations.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-02-13

#![no_std]
#![deny(missing_docs,)]
#![feature(allocator_api, type_alias_impl_trait, trusted_len, inplace_iteration, const_fn,)]

extern crate alloc;

#[doc(hidden,)]
pub mod array;
mod rank;
mod iter;
mod iter_mut;
mod into_iter;

pub use self::{array::Array, rank::*, iter::*, iter_mut::*, into_iter::*,};
#[doc(hidden,)]
pub use alloc::vec;

/// A helper macro for constructing [`Array`](crate::Array)s.
/// 
/// ```rust
/// # #[macro_use] extern crate jagged_array; fn main() {
/// use jagged_array::*;
/// 
/// assert_eq!(array![] as Array<()>, Array::new());
/// # }
/// ```
/// 
/// ```rust
/// # #[macro_use] extern crate jagged_array; fn main() {
/// use jagged_array::*;
/// 
/// let array = array![[2, 3]; 1, 2, 3, 4, 5];
/// assert_eq!(array[[1, 1]], 4);
/// # }
/// ```
/// 
/// ```rust
/// # #[macro_use] extern crate jagged_array; fn main() {
/// use jagged_array::*;
/// 
/// assert_eq!(array![1; [2, 3]], array![[2, 3]; 1, 1, 1, 1, 1]);
/// # }
/// ```
#[macro_export]
macro_rules! array {
  () => (Array::new());
  ($val:expr; [$($dim:expr),+]) => ($crate::array::from_elem($val, $crate::vec![$($dim,)+],));
  ([$($dim:expr),+]; $($val:expr),+ $(,)*) => ($crate::array::from_parts($crate::vec![$($dim,)+], $crate::vec![$($val,)+],));
}
