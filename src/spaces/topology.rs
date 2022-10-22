use super::{FiniteSet, FiniteSetIterator, Set};
use crate::core::arrays::DataHold;
use crate::core::types::Fe2O3SizeType;
use std::clone::Clone;
use std::iter::Iterator;
use std::ops::Fn;
use std::vec::Vec;

//--------------------------------------------------------------------------------------------------
// # Topology Traits
//--------------------------------------------------------------------------------------------------
/// A trait for topology bases
///
/// A topology is by nature a very large set being the sets of all open sets of a space. Holding a
/// topology is much too costly in terms of memory. Holding a minimal basis for a topology (i.e. a
/// minimal set of sets from which the entire topology may be constructed using union, intersection
/// and subsetting) is doable and sufficient.
///
/// One needs only implement one method `get_element` which retrieves an element of the topology basis.
pub trait TopologyBasis {
    type SetHandleT: Clone;
    type SubSetHandleT;
    /// Get a cell from the topology basis
    fn get_element(
        &self,
        handle: Self::SubSetHandleT,
    ) -> Option<DataHold<Self::SetHandleT, Vec<usize>>>;
}

//--------------------------------------------------------------------------------------------------
// # Implicit Topology Section
//--------------------------------------------------------------------------------------------------
/// A low memory implicit topology basis
///
/// This object represents a topology basis that can be encoded in a single function. This means that
/// there is some underlying structure in the topology that can be leveraged to construct the basis on
/// the fly instead of storing it in memory.
///
/// Typically, the topology of a structured grid is well suited to this type of description.
pub struct ImplicitTopologyBasis<HandleT: Clone, Closure>
where
    Closure: Fn(usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    /// The function descibing the topology basis of the space.
    mapper: Closure,
    cardinality: Fe2O3SizeType,
}

impl<HandleT: Clone, Closure> ImplicitTopologyBasis<HandleT, Closure>
where
    Closure: Fn(usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    /// A basic constructor taking am implicit mapping
    pub fn new(size: Fe2O3SizeType, map: Closure) -> Self {
        ImplicitTopologyBasis {
            cardinality: size,
            mapper: map,
        }
    }
    /// return an iterator over the topology basis
    pub fn iter(&self) -> ImplicitTopologyBasisIterator<HandleT, Closure> {
        ImplicitTopologyBasisIterator::new(&self)
    }
}

impl<HandleT: Clone, Closure> TopologyBasis for ImplicitTopologyBasis<HandleT, Closure>
where
    Closure: Fn(usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    type SetHandleT = HandleT;
    type SubSetHandleT = usize;
    /// Get a cell from the topology basis
    fn get_element(&self, handle: Self::SubSetHandleT) -> Option<DataHold<HandleT, Vec<usize>>> {
        let condition = match self.cardinality {
            Fe2O3SizeType::Finite(card) => handle < card,
            _ => false,
        };
        if condition {
            (self.mapper)(handle)
        } else {
            None
        }
    }
}

impl<HandleT: Clone, Closure> Set for ImplicitTopologyBasis<HandleT, Closure>
where
    Closure: Fn(usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    /// Get the size of the basis
    fn cardinality(&self) -> Fe2O3SizeType {
        self.cardinality
    }
}

/// An iterator for looping over a finite sized ImplicitTopologyBasis
pub struct ImplicitTopologyBasisIterator<'a, HandleT: Clone, Closure>
where
    Closure: Fn(usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    basis: &'a ImplicitTopologyBasis<HandleT, Closure>,
    top_handle: usize,
}

impl<'a, HandleT: Clone, Closure> ImplicitTopologyBasisIterator<'a, HandleT, Closure>
where
    Closure: Fn(usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    /// Constructor taking in the basis to iterate over
    pub fn new(new_basis: &'a ImplicitTopologyBasis<HandleT, Closure>) -> Self {
        match new_basis.cardinality() {
            Fe2O3SizeType::Finite(_) => (),
            _ => panic!("Cannot iterate over an infinite implicit topology basis"),
        };
        Self {
            basis: new_basis,
            top_handle: 0,
        }
    }
}

impl<'a, HandleT: Clone, Closure> Iterator for ImplicitTopologyBasisIterator<'a, HandleT, Closure>
where
    Closure: Fn(usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    type Item = DataHold<HandleT, Vec<usize>>;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.basis.get_element(self.top_handle);
        self.top_handle += 1;
        return res;
    }
}

//--------------------------------------------------------------------------------------------------
// # Explicit Topology Section
//--------------------------------------------------------------------------------------------------
/// Explicit topology basis stored in memory
///
/// This object represents a topology basis explicitly in a `FiniteSet`. Each "cell" is encoded in
/// an element of the set. These cells should have data that acts as handles/references to a
/// elements of a base set.
///
/// Unstructured grids should utilize this type of structure for their topology.
pub struct ExplicitTopologyBasis<'a, HandleT: Clone> {
    basis: FiniteSet<'a, HandleT>,
}

impl<'a, HandleT: Clone> ExplicitTopologyBasis<'a, HandleT> {
    /// A basic constructor taking a finite set as input
    pub fn new(new_basis: FiniteSet<'a, HandleT>) -> Self {
        ExplicitTopologyBasis { basis: new_basis }
    }
    /// Get an iterator over the cells
    pub fn iter(&'a self) -> FiniteSetIterator<'a, HandleT> {
        self.basis.iter()
    }
}

impl<'a, HandleT: Clone> TopologyBasis for ExplicitTopologyBasis<'a, HandleT> {
    type SetHandleT = HandleT;
    type SubSetHandleT = usize;
    /// Get a cell from the topology basis
    fn get_element(
        &self,
        handle: Self::SubSetHandleT,
    ) -> Option<DataHold<Self::SetHandleT, Vec<usize>>> {
        self.basis.get_element(handle)
    }
}

impl<'a, HandleT: Clone> Set for ExplicitTopologyBasis<'a, HandleT> {
    /// Get the size of the basis
    fn cardinality(&self) -> Fe2O3SizeType {
        self.basis.cardinality()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::arrays::DataMix;
    use std::iter::zip;
    use std::ops::Range;

    fn id_topology(index: usize) -> Option<DataHold<usize, Vec<usize>>> {
        Some(DataHold::new(vec![index], vec![1]))
    }

    #[test]
    fn test_create_implicit_topology_basis() {
        let top = ImplicitTopologyBasis::new(Fe2O3SizeType::Finite(10), id_topology);
        assert_eq!(
            top.cardinality(),
            Fe2O3SizeType::Finite(10),
            "Cardinality of implicit topology basis does not check out"
        )
    }

    #[test]
    fn test_get_element_implicit_topology_basis() {
        let top = ImplicitTopologyBasis::new(Fe2O3SizeType::Finite(10), id_topology);
        let el = top.get_element(4).unwrap();
        assert_eq!(
            el[0], 4,
            "get_element not functioning for ImplicitTopologyBasis"
        );
    }

    #[test]
    fn test_iter_implicit_topology_basis() {
        let top = ImplicitTopologyBasis::new(Fe2O3SizeType::Finite(10), id_topology);
        let range: Range<usize> = 0..10;
        for (i_el, el) in zip(range, top.iter()) {
            assert_eq!(i_el, el[0], "ImplicitTopologyBasisIterator looks broken");
        }
    }

    #[test]
    fn test_create_explicit_topology_basis() {
        let set = FiniteSet {
            elements: DataMix::Hold(DataHold::new([0..10].to_vec(), vec![5, 2])),
        };
        let top = ExplicitTopologyBasis::new(set);
        assert_eq!(
            top.cardinality(),
            Fe2O3SizeType::Finite(5),
            "Cardinality of explicit topology basis does not check out"
        )
    }

    #[test]
    fn test_get_element_explicit_topology_basis() {
        let set = FiniteSet {
            elements: DataMix::Hold(DataHold::new((0..10).collect(), vec![5, 2])),
        };
        let top = ExplicitTopologyBasis::new(set);
        let el = top.get_element(2).unwrap();
        assert_eq!(
            el[0], 4,
            "Get element of explicit topology basis failed on first check"
        );
        assert_eq!(
            el[1], 5,
            "Get element of explicit topology basis failed on second check"
        );
    }
}
