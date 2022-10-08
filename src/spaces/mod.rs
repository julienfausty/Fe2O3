pub mod sets;

pub mod topology;

pub use sets::{FiniteSet, FiniteSetIterator, InfiniteSet, Set};

pub enum NumberSet {
    Naturals(InfiniteSet),
    Integers(InfiniteSet),
    Rationals(InfiniteSet),
    Reals(InfiniteSet),
    EmptyNumberSet,
}
