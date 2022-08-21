// Here we define the basic blocks that we need in our library and how we're going to cut them up
// They are ordered by some hierarchy of dependence (geometry depends on core and assembly will depend
// on geometry etc.)

// module holding basic blocks we might need to share between other modules
pub mod core;

// module implementing data structures for storing the geometry of the problem
pub mod geometry;
// module implementing the linear or non-linear algebra tools for solving
pub mod algebra;

// module implementing the assembly of the physical problem into a discretized
// solvable system
pub mod assembly;
