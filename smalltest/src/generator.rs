use std::iter;

use itertools::*;

use crate::fair_flatten::*;

pub trait Generator: Iterator {
    fn or<T: Generator>(self, other: T) -> Interleave<Self, T::IntoIter>
    where
        Self: Sized,
        T: IntoIterator<Item = Self::Item>,
    {
        self.interleave(other)
    }

    fn choice<T>(self) -> FairFlatten<Self, T>
    where
        Self: Generator<Item = T> + Sized,
        T: Iterator,
    {
        FairFlatten::new(self)
    }

    fn and_then<T, F: Fn(Self::Item) -> T>(self, f: F) -> FairFlatten<std::iter::Map<Self, F>, T>
    where
        Self: Generator + Sized,
        T: Iterator,
    {
        FairFlatten::new(self.map(f))
    }
}

pub fn single<T>(t: T) -> iter::Once<T> {
    iter::once(t)
}

impl<T: ?Sized> Generator for T where T: Iterator {}

#[macro_export]
macro_rules! choose {
    ($($gen:expr),*) => {
        vec![$(Box::new($gen)),*].into_iter().choice()
    };
}
