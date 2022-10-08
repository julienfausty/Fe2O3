use super::data_traits::DataContainer;
use crate::default_tuple_data_container;
use std::convert::AsRef;
use std::ops::Deref;

//--------------------------------------------------------------------------------------------------
// # Structs
//--------------------------------------------------------------------------------------------------

/// Utility structure for wrapping multi-dimensional data without write access
///
/// A DataView is meant to be used when one wants to read data as a multi-dimensional array in a
/// non mutable way.
///
/// It takes two parameters to initialize: a reference to some contiguous data
/// array and a dimnesions array, in that order. The dimensions array has a length of the number of
/// dimensions in the array and each value is the size of that dimension. The convention is that
/// the last index changes the fastest (RowFirst). For example:
///
/// Data: v0 | v1 | v2 | ... | vn
/// Dimensions: d0 | d1
///
/// MultiDimViewOfData:
/// v0 | ... | v(d1-1)
/// v(d1) | ... | v(2d1 - 1)
///         ...
/// v((d0-1)d1) | ... | vn
pub struct DataView<'a, DataType, DimType: AsRef<[usize]>>(&'a [DataType], DimType);

impl<'a, DataType, DimType: AsRef<[usize]>> DataView<'a, DataType, DimType> {
    pub fn new(arr: &'a [DataType], shp: DimType) -> Self {
        Self(arr, shp)
    }
}

// Make the DataView behave like a reference to an array of DataType
impl<'a, DataType, DimType: AsRef<[usize]>> Deref for DataView<'a, DataType, DimType> {
    type Target = [DataType];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

// To enable an explicit cast of the data to [DataType]
impl<'a, DataType, DimType: AsRef<[usize]>> AsRef<[DataType]> for DataView<'a, DataType, DimType> {
    fn as_ref(&self) -> &[DataType] {
        self.0
    }
}

default_tuple_data_container!(DataView);

//--------------------------------------------------------------------------------------------------
// # Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_view_index() {
        let base_vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let dims: Vec<usize> = vec![8];
        let view = DataView(&base_vec, dims);
        assert_eq!(view[0], 0, "Indexing not working");
        assert_eq!(view[5], 5, "Indexing not working");
        let base_vec = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7];
        let dims: Vec<usize> = vec![8];
        let view = DataView(&base_vec, dims);
        assert_eq!(view[0], 0.0, "Indexing not working");
        assert_eq!(view[5], 0.5, "Indexing not working");
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    #[should_panic]
    fn test_data_view_bad_reshape() {
        let base_vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut view = DataView(&base_vec, vec![8]);
        view.reshape(vec![4, 5]);
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    fn test_data_view_reshape() {
        let base_vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut view = DataView(&base_vec, vec![8]);
        view.reshape(vec![4, 2]);
        assert_eq!(view.dimensions()[1], 2, "Did not reshape correctly")
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    #[should_panic]
    fn test_data_view_bad_multi_access() {
        let base_vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut view = DataView(&base_vec, vec![8]);
        view.reshape(vec![4, 2]);
        view.multi_index(vec![2, 3]);
    }

    //--------------------------------------------------------------------------------------------------
    #[test]
    fn test_data_view_multi_access() {
        let base_vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut view = DataView(&base_vec, vec![8]);
        view.reshape(vec![4, 2]);
        let val = 1;
        assert_eq!(
            view.multi_index(vec![0, 1]),
            &val,
            "multi index not working for (0, 1)"
        );
        let val = 6;
        assert_eq!(
            view.multi_index(vec![3, 0]),
            &val,
            "multi index not working for (3, 0)"
        );
        let val = 3;
        assert_eq!(
            view.multi_index(vec![1, 1]),
            &val,
            "multi index not working for (1, 1)"
        );
    }

    #[test]
    fn test_data_view_iteration() {
        let base_vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let view = DataView(&base_vec, vec![8]);
        for (iv, val) in view.iter().enumerate() {
            assert_eq!(val, &iv, "Failed iteration on value {}", iv);
        }
    }
}
