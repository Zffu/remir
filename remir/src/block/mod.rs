//! Definitions for blocks in the Remir IR representation

use std::collections::{HashMap, HashSet};

use crate::{block::vars::BlockVariable, insts::Instruction, module::Module, values::BaseSSAValue};

pub mod resolver;
pub mod vars;

/// Represents a reference to a [`Block`]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BlockReference {
    /// The name of the block
    pub name: String,

    /// The internal numerical ID of the block
    pub id: usize,
}

/// Represents a function block / branch.
#[derive(Clone)]
pub struct Block {
    /// The inner reference to feed.
    pub reference: BlockReference,

    pub instructions: Vec<BlockInstruction>,

    pub variables: HashMap<String, BlockVariable>,

    /// The origins of the block.
    /// The blocks that are leading to this block
    pub origins: HashSet<BlockReference>,

    /// The destinations of the block
    /// The blocks that the block is leading to
    pub destinations: HashSet<BlockReference>,
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
    ///     
    /// **Warn: Variable grabbing is only done if explicitly said (eg: using [`Block::grab_variables`]
    pub fn new(reference: BlockReference) -> Self {
        Self {
            reference,
            instructions: vec![],
            variables: HashMap::new(),
            origins: HashSet::new(),
            destinations: HashSet::new(),
        }
    }

    /// Grabs the block's variables.
    ///
    /// This is useful when making blocks that share the same function.
    ///
    /// **Warn: Variable grabbing is only done if explicitly said (eg: using [`Block::grab_variables`])
    pub fn grab_variables(&mut self, module: &Module, source: BlockReference) {
        let block = &module.blocks[source.id];

        for var in &block.variables {
            self.variables.insert(var.0.clone(), var.1.clone());
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
