pub mod data_traits;

pub mod data_view;

pub mod data_wrap;

pub mod data_hold;

pub use data_hold::DataHold;
pub use data_traits::{DataAllocator, DataContainer, DataMutator};
pub use data_view::DataView;
pub use data_wrap::DataWrap;
use std::clone::Clone;

pub enum DataMix<'a, DataType: Clone> {
    View(DataView<'a, DataType, Vec<usize>>),
    Wrap(DataWrap<'a, DataType, Vec<usize>>),
    Hold(DataHold<DataType, Vec<usize>>),
}

use DataMix::{Hold, View, Wrap};

impl<'a, DataType: Clone> DataContainer<DataType, Vec<usize>> for DataMix<'a, DataType> {
    fn dimensions(&self) -> &Vec<usize> {
        match &self {
            View(v) => v.dimensions(),
            Wrap(w) => w.dimensions(),
            Hold(h) => h.dimensions(),
        }
    }
    fn reshape(&mut self, newshape: Vec<usize>) {
        match self {
            View(v) => v.reshape(newshape),
            Wrap(w) => w.reshape(newshape),
            Hold(h) => h.reshape(newshape),
        }
    }
    fn flat_index(&self, multi: Vec<usize>) -> usize {
        match &self {
            View(v) => v.flat_index(multi),
            Wrap(w) => w.flat_index(multi),
            Hold(h) => h.flat_index(multi),
        }
    }
    fn multi_index(&self, multi: Vec<usize>) -> &DataType {
        match &self {
            View(v) => v.multi_index(multi),
            Wrap(w) => w.multi_index(multi),
            Hold(h) => h.multi_index(multi),
        }
    }
}
