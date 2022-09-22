use crate::{default_tuple_data_container, default_tuple_data_mutator};
use std::convert::{AsMut, AsRef};
use std::ops::{Deref, DerefMut};
use super::data_traits::{DataContainer, DataMutator};

//--------------------------------------------------------------------------------------------------
// # Structs
//--------------------------------------------------------------------------------------------------

/// Utility structure for wrapping multi-dimensional data with write access
///
/// A DataWrap is meant to be used when one wants to read data as a multi-dimensional array in a
/// mutable way but still not control allocation and sizing. 
///
/// Please see documentation of DataView for layout details.
pub struct DataWrap<'a, DataType, DimType: AsRef<[usize]>>(&'a mut [DataType], DimType);

// Make the DataWrap behave like a &[DataType]
impl<'a, DataType, DimType: AsRef<[usize]>> Deref for DataWrap<'a, DataType, DimType> {
    type Target = [DataType];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

// Make the DataWrap behave like a &mut [DataType]
impl<'a, DataType, DimType: AsRef<[usize]>> DerefMut for DataWrap<'a, DataType, DimType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

// To enable an explicit cast of the data to [DataType]
impl<'a, DataType, DimType: AsRef<[usize]>> AsRef<[DataType]> for DataWrap<'a, DataType, DimType> {
    fn as_ref(&self) -> &[DataType] {
        self.0
    }
}
impl<'a, DataType, DimType: AsRef<[usize]>> AsMut<[DataType]> for DataWrap<'a, DataType, DimType> {
    fn as_mut(&mut self) -> &mut [DataType] {
        self.0
    }
}

default_tuple_data_container!(DataWrap);
default_tuple_data_mutator!(DataWrap);

//--------------------------------------------------------------------------------------------------
// # Tests
//--------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_wrap_write() {
        let mut base = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut wrap = DataWrap(&mut base, vec![8]);
        wrap[4] = 0;
        assert_eq!(wrap[4], 0, "Change in index 4 was unsuccessful");
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    fn test_data_wrap_multi_index_write() {
        let mut base = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut wrap = DataWrap(&mut base, vec![2, 4]);
        *(wrap.multi_index_mut(vec![0, 3])) = 0;
        assert_eq!(wrap[3], 0, "Change in index 3 was unsuccessful");
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    fn test_data_wrap_iterator_write() {
        let mut base = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let mut wrap = DataWrap(&mut base, vec![2, 4]);
        for (iv, it) in wrap.iter_mut().enumerate() {
            *it = iv;
        }
        for (iv, it) in wrap.iter().enumerate() {
            assert_eq!(*it, iv, "Changes in mutable iterator were unsuccessful");
        }
    }
}
