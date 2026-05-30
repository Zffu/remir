//! Remir is an efficient, capable, and safe Middle Intermediate Representation for Calscin and other programming languages
//!
//! It uses a block-like representation with instructions and SSA values.

pub mod block;
pub mod builders;
pub mod insts;
pub mod module;
pub mod utils;
pub mod values;
pub mod writer;
