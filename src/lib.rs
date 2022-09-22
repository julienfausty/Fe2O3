//! Here we define the basic blocks that we need in our library and how we're going to cut them up
//!
//! They are ordered by some hierarchy of dependence (spaces depends on core and discretizations will depend
//! on spaces etc.)

// module holding basic blocks we might need to share between other modules
pub mod core;
// module holding abstract structures for representing infinite and finite mathematical spaces
pub mod spaces;

// module implementing data structures and algorithms for storing and assembling the discretized
// problem
pub mod discretizations;
// module implementing the linear or non-linear algebra tools for solving systems
pub mod solvers;

// module implementing recurring data pipelines while using the library
pub mod workflows;
