//! Definitions for blocks in the Remir IR representation

use std::collections::HashMap;

use crate::{insts::Instruction, values::BaseSSAValue};

pub mod vars;

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

    pub instructions: Vec<BlockInstruction>,

    pub values: HashMap<String>,
}

/// Represents instructions that are held in a [`Block`]
#[derive(Clone)]
pub struct BlockInstruction {
    pub instruction: Instruction,
    pub value: Option<BaseSSAValue>,
}

impl BlockReference {
    /// Creates a new [`BlockReference`]
    pub fn new(name: String, id: usize) -> Self {
        Self { name, id }
    }
}

impl Block {
    /// Creates a new [`Block`]
    pub fn new(reference: BlockReference) -> Self {
        Self {
            reference,
            instructions: vec![],
        }
    }
}

impl BlockInstruction {
    /// Creates a new [`BlockInstruction`]
    pub fn new(instruction: Instruction, value: Option<BaseSSAValue>) -> Self {
        Self { instruction, value }
    }

    pub fn get(&self) -> Result<BaseSSAValue, ()> {
        match &self.value {
            Option::None => Err(()),
            Option::Some(v) => Ok(v.clone()),
        }
    }
}
