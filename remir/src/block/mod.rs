//! Definitions for blocks in the Remir IR representation

/// Represents a reference to a [`Block`]
#[derive(Clone)]
pub struct BlockReference {
    /// The name of the block
    pub name: String,

    /// The internal numerical ID of the block
    pub id: usize,
}

/// Represents a function block / branch.
pub struct Block {
    /// The inner reference to feed.
    pub reference: BlockReference,
}

impl BlockReference {
    pub fn new(name: String, id: usize) -> Self {
        Self { name, id }
    }
}

impl Block {
    pub fn new(reference: BlockReference) -> Self {
        Self { reference }
    }
}
