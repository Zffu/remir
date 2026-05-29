//! Definitions for blocks in the Remir IR representation

#[derive(Clone)]
pub struct BlockReference {
    pub name: String,
    pub id: usize,
}
