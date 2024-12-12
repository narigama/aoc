use itertools::{Itertools, MultiProduct};

/// Rust version of Python's itertools.product().
/// It returns the cartesian product of the input iterables, and it is
/// semantically equivalent to `repeat` nested for loops.
///
/// # Arguments
///
/// * `it` - An iterator over a cloneable data structure
/// * `repeat` - Number of repetitions of the given iterator
pub fn product_repeat<I>(it: I, repeat: usize) -> MultiProduct<I>
  where
    I: Iterator + Clone,
    I::Item: Clone {
  std::iter::repeat(it)
    .take(repeat)
    .multi_cartesian_product()
}

pub trait ProductRepeat: Iterator + Clone
  where Self::Item: Clone {
  fn product_repeat(self, repeat: usize) -> MultiProduct<Self> {
    std::iter::repeat(self)
      .take(repeat)
      .multi_cartesian_product()
  }
}

impl<T: Iterator + Clone> ProductRepeat for T
  where T::Item: Clone {}