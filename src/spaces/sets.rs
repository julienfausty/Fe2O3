use crate::core::arrays::{DataContainer, DataMix};
use crate::core::types::Fe2O3SizeType;
use std::clone::Clone;

pub trait Set {
    fn cardinality(&self) -> Fe2O3SizeType;
}

pub struct InfiniteSet {}

impl Set for InfiniteSet {
    fn cardinality(&self) -> Fe2O3SizeType {
        Fe2O3SizeType::Infinity
    }
}

pub enum NumberSet {
    Naturals(InfiniteSet),
    Integers(InfiniteSet),
    Rationals(InfiniteSet),
    Reals(InfiniteSet),
    EmptyNumberSet,
}

pub struct FiniteSet<'a, BaseType: Clone> {
    elements: DataMix<'a, BaseType>,
}

impl<'a, BaseType: Clone> Set for FiniteSet<'a, BaseType> {
    fn cardinality(&self) -> Fe2O3SizeType {
        let card: usize = match &self.elements {
            DataMix::View(view) => view.dimensions()[0],
            DataMix::Wrap(wrap) => wrap.dimensions()[0],
            DataMix::Hold(hold) => hold.dimensions()[0],
        };
        Fe2O3SizeType::Finite(card)
    }
}
