//! Definitions for blocks in the Remir IR representation

use std::collections::{HashMap, HashSet};

use crate::{
    block::vars::BlockVariable, errs::RemirResult, insts::Instruction, module::Module, return_err,
    values::BaseSSAValue,
};

pub mod sync;
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

    /// The instructions contained within the block.
    pub instructions: Vec<BlockInstruction>,

    /// The variables contained within the block.
    /// Variables are optional and are simply for convenience.
    pub variables: HashMap<String, BlockVariable>,

    /// The origins of the block.
    /// The blocks that are leading to this block
    pub origins: HashSet<BlockReference>,

    /// The destinations of the block
    /// The blocks that the block is leading to
    pub destinations: HashSet<BlockReference>,

    /// The dependencies of the block
    /// The blocks that need to be loaded before this block
    pub dependencies: HashSet<BlockReference>,
}

/// Represents instructions that are held in a [`Block`]
#[derive(Clone)]
pub struct BlockInstruction {
    /// The actual instruction held by the [`BlockInstruction`]
    pub instruction: Instruction,

    /// The potential output value of the instruction
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
            dependencies: HashSet::new(),
        }
    }

    /// Appends a variable into the [`Block`]
    pub fn append_variable(&mut self, variable: BlockVariable) {
        self.variables.insert(variable.name.clone(), variable);
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

    /// Returns the held value of the [`BlockInstruction`] if there is one
    ///
    /// # Errors
    /// This function will error if the [`BlockInstruction`] doesn't contain any value
    ///
    pub fn get(&self) -> RemirResult<BaseSSAValue> {
        match &self.value {
            Option::None => {
                return_err!("Tried unwrapping a nonvalued BlockInstruction into a value")
            }
            Option::Some(v) => Ok(v.clone()),
        }
    }
}
