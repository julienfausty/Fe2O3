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
