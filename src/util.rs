use itertools::{Itertools, MultiProduct};

pub trait ProductRepeat: Iterator + Clone
where
    Self::Item: Clone,
{
    fn product_repeat(self, repeat: usize) -> MultiProduct<Self> {
        std::iter::repeat(self).take(repeat).multi_cartesian_product()
    }
}

impl<T> ProductRepeat for T
where
    T: Iterator + Clone,
    T::Item: Clone,
{
}
