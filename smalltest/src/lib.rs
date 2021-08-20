pub mod arbitrary;
pub mod fair_flatten;
pub mod generator;

pub use crate::arbitrary::*;
pub use crate::generator::*;

pub fn test<A: Arbitrary + core::fmt::Debug + Clone, F: Fn(A) -> bool>(f: F) {
    forall(A::any(), f)
}

pub fn forall<A: Clone + core::fmt::Debug, F: Fn(A) -> bool, I: Iterator<Item = A>>(i: I, f: F) {
    for a in i.take(1_000_000) {
        assert!(f(a.clone()), "Counter-example: {:?}", a)
    }
}

#[test]
fn basic_algebraic_properties() {
    test(|(x, y): (u8, u8)| x.checked_add(y) == y.checked_add(x));
}
