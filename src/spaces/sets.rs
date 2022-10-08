use crate::core::arrays::{DataContainer, DataHold, DataMix};
use crate::core::types::Fe2O3SizeType;
use std::clone::Clone;
use std::iter::Iterator;

pub trait Set {
    fn cardinality(&self) -> Fe2O3SizeType;
}

pub struct InfiniteSet {}

impl Set for InfiniteSet {
    fn cardinality(&self) -> Fe2O3SizeType {
        Fe2O3SizeType::Infinity
    }
}

pub struct FiniteSet<'a, BaseType: Clone> {
    pub elements: DataMix<'a, BaseType>,
}

impl<'a, BaseType: Clone> FiniteSet<'a, BaseType> {
    pub fn iter(&'a self) -> FiniteSetIterator<'a, BaseType> {
        FiniteSetIterator::new(&self)
    }
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

pub struct FiniteSetIterator<'a, BaseType: Clone> {
    set: &'a FiniteSet<'a, BaseType>,
    element_shape: Vec<usize>,
    element_size: usize,
    handle: usize,
    n_elements: usize,
    current_element: &'a [BaseType],
}

impl<'a, BaseType: Clone> FiniteSetIterator<'a, BaseType> {
    pub fn new(new_set: &'a FiniteSet<'a, BaseType>) -> Self {
        let n_elements = match new_set.cardinality() {
            Fe2O3SizeType::Finite(c) => c,
            _ => panic!("The cardinality of this finite set is not finite!"),
        };
        let element_size = match &new_set.elements {
            DataMix::View(v) => v.len() / n_elements,
            DataMix::Wrap(w) => w.len() / n_elements,
            DataMix::Hold(h) => h.len() / n_elements,
        };
        let mut el_shape: Vec<usize> = vec![0; element_size];
        if element_size == 1 {
            el_shape[0] = 1;
        } else {
            el_shape.copy_from_slice(&new_set.elements.dimensions()[1..]);
        }
        let current_el = match &new_set.elements {
            DataMix::View(v) => &v[..element_size],
            DataMix::Wrap(w) => &w[..element_size],
            DataMix::Hold(h) => &h[..element_size],
        };
        Self {
            set: new_set,
            element_shape: el_shape,
            element_size: element_size,
            handle: 0,
            n_elements: n_elements,
            current_element: current_el,
        }
    }
}

impl<'a, BaseType: Clone> Iterator for FiniteSetIterator<'a, BaseType> {
    type Item = DataHold<BaseType, Vec<usize>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.handle >= self.n_elements {
            return None;
        }
        let res = DataHold::new(self.current_element.to_vec(), self.element_shape.to_vec());
        self.handle += 1;
        let start_index = self.element_size * self.handle;
        self.current_element = match &self.set.elements {
            DataMix::View(v) => &v[start_index..start_index + self.element_size],
            DataMix::Wrap(w) => &w[start_index..start_index + self.element_size],
            DataMix::Hold(h) => &h[start_index..start_index + self.element_size],
        };
        Some(res)
    }
}
