use crate::generator::*;

pub trait Arbitrary: Eq + core::fmt::Debug {
    type Gen: Generator<Item = Self>;

    fn any() -> Self::Gen;
}

impl Arbitrary for u8 {
    type Gen = core::ops::Range<u8>;

    fn any() -> Self::Gen {
        u8::MIN..u8::MAX
    }
}

impl<A: Arbitrary, B: Arbitrary> Arbitrary for (A, B)
where
    A: 'static + Clone,
    B: 'static,
{
    type Gen = Box<dyn Generator<Item = (A, B)>>;

    fn any() -> Self::Gen {
        Box::new(A::any().and_then(|a| B::any().map(move |b| (a.clone(), b))))
    }
}

impl<A: Arbitrary, E: Arbitrary> Arbitrary for Result<A, E>
where
    A: 'static,
    E: 'static,
{
    type Gen = Box<dyn Generator<Item = Result<A, E>>>;

    fn any() -> Self::Gen {
        Box::new(A::any().map(Result::Ok).or(E::any().map(Result::Err)))
    }
}
