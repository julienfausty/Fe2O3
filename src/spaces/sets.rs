use crate::core::arrays::{DataContainer, DataHold, DataMix};
use crate::core::types::Fe2O3SizeType;
use std::clone::Clone;
use std::iter::Iterator;

/// A trait describing a set
///
/// The only thing that we can reliably demand of a set is its size (cardinality). As such, that is the only method structures should implement.
pub trait Set {
    /// the number of elements in the set
    fn cardinality(&self) -> Fe2O3SizeType;
}

/// A structure representing an infinite set
///
/// It has no members since the number of members would be infinite and, therefore, not fit into memory. It returns `Infinity` as its cardinality.
pub struct InfiniteSet {}

impl Set for InfiniteSet {
    /// should always return infinity
    fn cardinality(&self) -> Fe2O3SizeType {
        Fe2O3SizeType::Infinity
    }
}

/// A structure representing a finite set
///
/// The finite set is comprised of a bucket of elements of type `BaseType` stored in a public `DataMix`.
/// The first dimension of the data is the cardinality and the other dimensions are intrinsic to the type
/// of data comprising the set.
pub struct FiniteSet<'a, BaseType: Clone> {
    pub elements: DataMix<'a, BaseType>,
}

impl<'a, BaseType: Clone> FiniteSet<'a, BaseType> {
    /// Returns an iterator over the elements of the set
    pub fn iter(&'a self) -> FiniteSetIterator<'a, BaseType> {
        FiniteSetIterator::new(self)
    }
    /// Returns an element at the position described by the `handle` integer parameter
    pub fn get_element(&self, handle: usize) -> Option<DataHold<BaseType, Vec<usize>>> {
        let card = match self.cardinality() {
            Fe2O3SizeType::Finite(c) => c,
            _ => panic!("Cardinality of finite set is not finite!"),
        };
        if handle > card {
            return None;
        }
        let element_size = match &self.elements {
            DataMix::View(v) => v.len() / v.dimensions()[0],
            DataMix::Wrap(w) => w.len() / w.dimensions()[0],
            DataMix::Hold(h) => h.len() / h.dimensions()[0],
        };
        let start_index = element_size * handle;
        let mut el_shape: Vec<usize> = vec![0; &self.elements.dimensions().len() - 1];
        if element_size == 1 {
            el_shape[0] = 1;
        } else {
            el_shape.copy_from_slice(&self.elements.dimensions()[1..]);
        }
        let slice = match &self.elements {
            DataMix::View(v) => &v[start_index..start_index + element_size],
            DataMix::Wrap(w) => &w[start_index..start_index + element_size],
            DataMix::Hold(h) => &h[start_index..start_index + element_size],
        };
        Some(DataHold::new(slice.to_vec(), el_shape))
    }
}

impl<'a, BaseType: Clone> Set for FiniteSet<'a, BaseType> {
    /// Returns the finite size of the set
    fn cardinality(&self) -> Fe2O3SizeType {
        let card: usize = match &self.elements {
            DataMix::View(view) => view.dimensions()[0],
            DataMix::Wrap(wrap) => wrap.dimensions()[0],
            DataMix::Hold(hold) => hold.dimensions()[0],
        };
        Fe2O3SizeType::Finite(card)
    }
}

/// An iterator over the a `FiniteSet`
///
/// Can be used to iterate over the elements of a `FiniteSet`
pub struct FiniteSetIterator<'a, BaseType: Clone> {
    /// A reference to the set
    set: &'a FiniteSet<'a, BaseType>,
    /// The shaoe of one element
    element_shape: Vec<usize>,
    /// The size (in number of `BaseType`s) of one element
    element_size: usize,
    /// The next position of the iterator in the set
    handle: usize,
    /// The cardinality of the set
    n_elements: usize,
    /// A view of the current element of the set (at the position described by `handle`)
    current_element: &'a [BaseType],
}

impl<'a, BaseType: Clone> FiniteSetIterator<'a, BaseType> {
    /// Constructor for the iterator
    pub fn new(new_set: &'a FiniteSet<'a, BaseType>) -> Self {
        let n_el = match new_set.cardinality() {
            Fe2O3SizeType::Finite(c) => c,
            _ => panic!("The cardinality of this finite set is not finite!"),
        };
        let el_size = match &new_set.elements {
            DataMix::View(v) => v.len() / n_el,
            DataMix::Wrap(w) => w.len() / n_el,
            DataMix::Hold(h) => h.len() / n_el,
        };
        let mut el_shape: Vec<usize> = vec![0; &new_set.elements.dimensions().len() - 1];
        if el_size == 1 {
            el_shape[0] = 1;
        } else {
            el_shape.copy_from_slice(&new_set.elements.dimensions()[1..]);
        }
        let current_el = match &new_set.elements {
            DataMix::View(v) => &v[..el_size],
            DataMix::Wrap(w) => &w[..el_size],
            DataMix::Hold(h) => &h[..el_size],
        };
        Self {
            set: new_set,
            element_shape: el_shape,
            element_size: el_size,
            handle: 0,
            n_elements: n_el,
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
        let start_index = self.element_size * self.handle;
        self.current_element = match &self.set.elements {
            DataMix::View(v) => &v[start_index..start_index + self.element_size],
            DataMix::Wrap(w) => &w[start_index..start_index + self.element_size],
            DataMix::Hold(h) => &h[start_index..start_index + self.element_size],
        };
        let res = DataHold::new(self.current_element.to_vec(), self.element_shape.to_vec());
        self.handle += 1;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_infinite_set() {
        let inf_set = InfiniteSet {};
        assert_eq!(
            inf_set.cardinality(),
            Fe2O3SizeType::Infinity,
            "Cardinality of infinite set is not equal to infinity!"
        );
    }

    #[test]
    fn test_create_finite_set() {
        let set = FiniteSet {
            elements: DataMix::Hold(DataHold::new(vec![0, 1, 2, 3, 4, 5], vec![3, 2])),
        };
        assert_eq!(
            set.cardinality(),
            Fe2O3SizeType::Finite(3),
            "Error in getting cardinality of finite set"
        )
    }

    #[test]
    fn test_get_element_finite_set() {
        let set = FiniteSet {
            elements: DataMix::Hold(DataHold::new(vec![0, 1, 2, 3, 4, 5], vec![3, 2])),
        };
        let el = set.get_element(1).unwrap();
        assert_eq!(
            el.dimensions().len(),
            1,
            "Length of dimensions of set element is wrong"
        );
        assert_eq!(
            el.dimensions()[0],
            2,
            "Value of dimensions of set element is wrong"
        );
        assert_eq!(el.len(), 2, "Length of element is wrong");
        assert_eq!(el[0], 2, "First value of element is wrong");
        assert_eq!(el[1], 3, "Second value of element is wrong");
    }

    #[test]
    fn test_iterator_finite_set() {
        let set = FiniteSet {
            elements: DataMix::Hold(DataHold::new(vec![0, 1, 2, 3, 4, 5], vec![3, 2])),
        };
        let mut iel = 0;
        for el in set.iter() {
            assert_eq!(el[0], iel, "First value in iteration {} is wrong", iel);
            assert_eq!(el[1], iel + 1, "Second value in iteration {} is wrong", iel);
            iel += 2;
        }
        assert_eq!(iel, 6, "Iterator did not iterate through entire set");
    }
}
