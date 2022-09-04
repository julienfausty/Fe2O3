//--------------------------------------------------------------------------------------------------
// # Traits
//--------------------------------------------------------------------------------------------------
/// A trait for implementing read only operations on data
pub trait DataContainer<DataType, DimType: AsRef<[usize]>> {
    /// Get the multi-dimensions of the data array
    fn dimensions(&self) -> &DimType;
    /// Reshape the data to the given dimensions
    fn reshape(&mut self, newshape: DimType);
    /// Get the flat index from the multi index given the current shape
    fn flat_index(&self, mindex: DimType) -> usize;
    /// Retrieve the value at a multi-index
    fn multi_index(&self, mindex: DimType) -> &DataType;
}

//--------------------------------------------------------------------------------------------------
///A trait for implementing write operations on data
pub trait DataMutator<DataType, DimType: AsRef<[usize]>> {
    fn multi_index_mut(&mut self, mindex: DimType) -> &mut DataType;
}

//--------------------------------------------------------------------------------------------------
///A trait for implementing full allocation on data
pub trait DataAllocator<DataType, DimType: AsRef<[usize]>> {
    fn resize(&mut self, newshape: DimType, value: DataType);
}

//--------------------------------------------------------------------------------------------------
// # Macros
//--------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! default_tuple_data_container {
    ($struct_name: ident) => {
        use std::iter::zip;
        impl<'a, DataType, DimType: AsRef<[usize]>> DataContainer<DataType, DimType> for $struct_name<'a, DataType, DimType> {
            fn dimensions(&self) -> &DimType {
                &self.1
            }
            fn reshape(&mut self, newshape: DimType) {
                fn comp_coherency(shape: &[usize], comps: usize) -> bool {
                    let tot_comps: usize = shape.iter().product();
                    tot_comps == comps
                }
                assert!(
                    comp_coherency(newshape.as_ref(), self.0.len()),
                    "Tried to reshape to uncompatible shape"
                );
                self.1 = newshape;
            }
            fn flat_index(&self, index: DimType) -> usize {
                assert!(
                    index.as_ref().len() == self.1.as_ref().len(),
                    "Tried to multi index a DataView with an index having a different number of dimensions"
                );
                fn idx_coherency(s: &[usize], i: &[usize]) -> bool {
                    for (size, idx) in zip(s.iter(), i.iter()) {
                        if idx >= size {
                            return false;
                        }
                    }
                    true
                }
                assert!(
                    idx_coherency(self.1.as_ref(), index.as_ref()),
                    "Tried multi indexing with an index larger then the dimensions"
                );
                let mut flat_index: usize = 0;
                let mut count: usize = 1;
                for dim in index.as_ref().iter() {
                    flat_index += dim;
                    if (self.1.as_ref().len() - count) != 0 {
                        flat_index *= self.1.as_ref()[count];
                        count += 1;
                    }
                }
                flat_index
            }
            fn multi_index(&self, index: DimType) -> &DataType {
                &self.0[self.flat_index(index)]
            }
        }
    }
}

//--------------------------------------------------------------------------------------------------
#[macro_export]
macro_rules! default_tuple_data_mutator {
    ($struct_name: ident) => {
        impl<'a, DataType, DimType: AsRef<[usize]>> DataMutator<DataType, DimType>
            for $struct_name<'a, DataType, DimType>
        {
            fn multi_index_mut(&mut self, index: DimType) -> &mut DataType {
                &mut self.0[self.flat_index(index)]
            }
        }
    };
}
