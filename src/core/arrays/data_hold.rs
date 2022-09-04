use super::data_traits::{DataAllocator, DataContainer, DataMutator};
use std::clone::Clone;
use std::convert::{AsMut, AsRef};
use std::iter::zip;
use std::ops::{Deref, DerefMut};

//--------------------------------------------------------------------------------------------------
// # Structs
//--------------------------------------------------------------------------------------------------

/// Utility structure for wrapping multi-dimensional data with write access and allocation
/// capabibilities
///
/// A DataHold is meant to be used when one wants to read data as a multi-dimensional array in a
/// mutable way and control allocation and sizing.
///
/// Please see documentation of DataView for layout details.
pub struct DataHold<DataType: Clone, DimType: AsRef<[usize]>>(Vec<DataType>, DimType);

// Make the DataHold behave like a &[DataType]
impl<DataType: Clone, DimType: AsRef<[usize]>> Deref for DataHold<DataType, DimType> {
    type Target = [DataType];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Make the DataHold behave like a &mut [DataType]
impl<DataType: Clone, DimType: AsRef<[usize]>> DerefMut for DataHold<DataType, DimType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// To enable an explicit cast of the data to [DataType]
impl<DataType: Clone, DimType: AsRef<[usize]>> AsRef<[DataType]> for DataHold<DataType, DimType> {
    fn as_ref(&self) -> &[DataType] {
        &self.0
    }
}
impl<DataType: Clone, DimType: AsRef<[usize]>> AsMut<[DataType]> for DataHold<DataType, DimType> {
    fn as_mut(&mut self) -> &mut [DataType] {
        &mut self.0
    }
}

impl<DataType: Clone, DimType: AsRef<[usize]>> DataContainer<DataType, DimType>
    for DataHold<DataType, DimType>
{
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

impl<DataType: Clone, DimType: AsRef<[usize]>> DataMutator<DataType, DimType>
    for DataHold<DataType, DimType>
{
    fn multi_index_mut(&mut self, index: DimType) -> &mut DataType {
        let flat = self.flat_index(index);
        &mut self.0[flat]
    }
}

impl<DataType: Clone, DimType: AsRef<[usize]>> DataAllocator<DataType, DimType>
    for DataHold<DataType, DimType>
{
    fn resize(&mut self, newshape: DimType, value: DataType) {
        let tot_comps: usize = newshape.as_ref().iter().product();
        self.0.resize(tot_comps, value);
        self.reshape(newshape);
    }
}

//--------------------------------------------------------------------------------------------------
// # Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_hold_index() {
        let hold = DataHold(vec![0, 1, 2, 3, 4, 5, 6, 7], vec![8]);
        assert_eq!(hold[0], 0, "Indexing not working");
        assert_eq!(hold[5], 5, "Indexing not working");
        let hold = DataHold(vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7], vec![8]);
        assert_eq!(hold[0], 0.0, "Indexing not working");
        assert_eq!(hold[5], 0.5, "Indexing not working");
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    #[should_panic]
    fn test_data_hold_bad_reshape() {
        let mut hold = DataHold(vec![0, 1, 2, 3, 4, 5, 6, 7], vec![8]);
        hold.reshape(vec![4, 5]);
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    fn test_data_hold_reshape() {
        let mut hold = DataHold(vec![0, 1, 2, 3, 4, 5, 6, 7], vec![8]);
        hold.reshape(vec![4, 2]);
        assert_eq!(hold.dimensions()[1], 2, "Did not reshape correctly")
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    #[should_panic]
    fn test_data_hold_bad_multi_access() {
        let mut hold = DataHold(vec![0, 1, 2, 3, 4, 5, 6, 7], vec![8]);
        hold.reshape(vec![4, 2]);
        hold.multi_index(vec![2, 3]);
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    fn test_data_hold_multi_access() {
        let mut hold = DataHold(vec![0, 1, 2, 3, 4, 5, 6, 7], vec![8]);
        hold.reshape(vec![4, 2]);
        let val = 1;
        assert_eq!(
            hold.multi_index(vec![0, 1]),
            &val,
            "multi index not working for (0, 1)"
        );
        let val = 6;
        assert_eq!(
            hold.multi_index(vec![3, 0]),
            &val,
            "multi index not working for (3, 0)"
        );
        let val = 3;
        assert_eq!(
            hold.multi_index(vec![1, 1]),
            &val,
            "multi index not working for (1, 1)"
        );
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    fn test_data_hold_iteration() {
        let hold = DataHold(vec![0, 1, 2, 3, 4, 5, 6, 7], vec![8]);
        for (iv, val) in hold.iter().enumerate() {
            assert_eq!(val, &iv, "Failed iteration on value {}", iv);
        }
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    fn test_data_hold_write() {
        let mut hold = DataHold(vec![0, 1, 2, 3, 4, 5, 6, 7], vec![8]);
        hold[4] = 0;
        assert_eq!(hold[4], 0, "Change in index 4 was unsuccessful");
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    fn test_data_hold_multi_index_write() {
        let mut hold = DataHold(vec![0, 1, 2, 3, 4, 5, 6, 7], vec![2, 4]);
        *(hold.multi_index_mut(vec![0, 3])) = 0;
        assert_eq!(hold[3], 0, "Change in index 3 was unsuccessful");
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    fn test_data_hold_iterator_write() {
        let mut hold = DataHold(vec![0, 0, 0, 0, 0, 0, 0, 0], vec![2, 4]);
        for (iv, it) in hold.iter_mut().enumerate() {
            *it = iv;
        }
        for (iv, it) in hold.iter().enumerate() {
            assert_eq!(*it, iv, "Changes in mutable iterator were unsuccessful");
        }
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    fn test_data_hold_resize() {
        let mut hold: DataHold<i32, Vec<usize>> = DataHold(vec![], vec![]);
        hold.resize(vec![6, 3, 5], 0);
        assert_eq!(hold.len(), 6*3*5, "Did not resize data correctly");
        let dims = vec![6,3,5];
        for (iv, it) in zip(hold.dimensions().iter(), dims.iter()) {
            assert_eq!(iv, it, "Did not set dimensions correctly during resize");
        }
    }
}
