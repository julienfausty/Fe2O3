use super::{FiniteSet, FiniteSetIterator};
use crate::core::arrays::DataHold;
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
        handle: Self::SubSetHandleT,
    ) -> Option<DataHold<Self::SetHandleT, Vec<usize>>>;
}

//--------------------------------------------------------------------------------------------------
// # Implicit Topology Section
//--------------------------------------------------------------------------------------------------

pub struct ImplicitTopologyBasis<HandleT: Clone, Closure>
where
    Closure: Fn(usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    mapper: Closure,
}

impl<HandleT: Clone, Closure> ImplicitTopologyBasis<HandleT, Closure>
where
    Closure: Fn(usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    pub fn new(map: Closure) -> Self {
        ImplicitTopologyBasis { mapper: map }
    }
}

impl<HandleT: Clone, Closure> TopologyBasis for ImplicitTopologyBasis<HandleT, Closure>
where
    Closure: Fn(usize) -> Option<DataHold<HandleT, Vec<usize>>>,
{
    type SetHandleT = HandleT;
    type SubSetHandleT = usize;
    fn get_element(&self, handle: Self::SubSetHandleT) -> Option<DataHold<HandleT, Vec<usize>>> {
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
        handle: Self::SubSetHandleT,
    ) -> Option<DataHold<Self::SetHandleT, Vec<usize>>> {
        self.basis.get_element(handle)
    }
}
