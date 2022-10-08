use super::{FiniteSet, FiniteSetIterator, Set};
use crate::core::arrays::{DataContainer, DataHold, DataMix};
use crate::core::types::Fe2O3SizeType;
use std::ops::Fn;
use std::vec::Vec;

//--------------------------------------------------------------------------------------------------
// # Topology Traits
//--------------------------------------------------------------------------------------------------
pub trait TopologyBasis {
    type SetHandleT: Clone;
    type SubSetHandleT;
    fn get_element(
        &self,
        handle: &Self::SubSetHandleT,
    ) -> Option<DataHold<Self::SetHandleT, Vec<usize>>>;
}

//--------------------------------------------------------------------------------------------------
// # Implicit Topology Section
//--------------------------------------------------------------------------------------------------

pub struct ImplicitTopologyBasis<HandleT: Clone, Closure>
where
    Closure: Fn(&usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    mapper: Closure,
}

impl<HandleT: Clone, Closure> ImplicitTopologyBasis<HandleT, Closure>
where
    Closure: Fn(&usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    pub fn new(map: Closure) -> Self {
        ImplicitTopologyBasis { mapper: map }
    }
}

impl<HandleT: Clone, Closure> TopologyBasis for ImplicitTopologyBasis<HandleT, Closure>
where
    Closure: Fn(&usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    type SetHandleT = HandleT;
    type SubSetHandleT = usize;
    fn get_element(&self, handle: &Self::SubSetHandleT) -> Option<DataHold<HandleT, Vec<usize>>> {
        (self.mapper)(handle)
    }
}

//--------------------------------------------------------------------------------------------------
// # Explicit Topology Section
//--------------------------------------------------------------------------------------------------
pub struct ExplicitTopologyBasis<'a, HandleT: Clone> {
    basis: FiniteSet<'a, HandleT>,
}

impl<'a, HandleT: Clone> ExplicitTopologyBasis<'a, HandleT> {
    pub fn new(new_basis: FiniteSet<'a, HandleT>) -> Self {
        ExplicitTopologyBasis { basis: new_basis }
    }
    pub fn iter(&'a self) -> FiniteSetIterator<'a, HandleT> {
        self.basis.iter()
    }
}

impl<'a, HandleT: Clone> TopologyBasis for ExplicitTopologyBasis<'a, HandleT> {
    type SetHandleT = HandleT;
    type SubSetHandleT = usize;
    fn get_element(
        &self,
        handle: &Self::SubSetHandleT,
    ) -> Option<DataHold<Self::SetHandleT, Vec<usize>>> {
        let card = match self.basis.cardinality() {
            Fe2O3SizeType::Finite(c) => c,
            _ => panic!("Cardinality of finite set is not finite!"),
        };
        if handle > &card {
            return None;
        }
        let element_size = match &self.basis.elements {
            DataMix::View(v) => v.len() / v.dimensions()[0],
            DataMix::Wrap(w) => w.len() / w.dimensions()[0],
            DataMix::Hold(h) => h.len() / h.dimensions()[0],
        };
        let start_index = element_size * handle;
        let mut el_shape: Vec<usize> = vec![0; element_size];
        if element_size == 1 {
            el_shape[0] = 1;
        } else {
            el_shape.copy_from_slice(&self.basis.elements.dimensions()[1..]);
        }
        let slice = match &self.basis.elements {
            DataMix::View(v) => &v[start_index..start_index + element_size],
            DataMix::Wrap(w) => &w[start_index..start_index + element_size],
            DataMix::Hold(h) => &h[start_index..start_index + element_size],
        };
        Some(DataHold::new(slice.to_vec(), el_shape))
    }
}
