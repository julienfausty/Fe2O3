//! Fe2O3: a finite element rust library


// Here we define the basic blocks that we need in our library and how we're going to cut them up
//
// They are ordered by some hierarchy of dependence (spaces depends on core and discretizations will depend
// on spaces etc.)

/// Module holding basic blocks we might need to share between other modules
pub mod core;
/// Module holding abstract structures for representing infinite and finite mathematical spaces
pub mod spaces;

/// Module implementing data structures and algorithms for storing and assembling the discretized
/// problem
pub mod discretizations;
/// Module implementing the linear or non-linear algebra tools for solving systems
pub mod solvers;

/// Module implementing recurring data pipelines while using the library
pub mod workflows;
