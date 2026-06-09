//! A module is a storage for instructions, functions and blocks. It holds the context of the entire Remir library.

use std::collections::{HashMap, HashSet};

use crate::{
    block::{Block, BlockReference, sync::VariableSynchronizer},
    errs::RemirResult,
    func::{Function, FunctionReference},
    return_err,
    values::ValueType,
};

/// A module represents the base of a program.
/// It it recomended to use a module per file.
///
/// Every single module has it's own functions, blocks and instruction writer.
pub struct Module {
    /// The name of the module
    pub name: String,

    /// The blocks contained in the module
    pub blocks: Vec<Block>,

    /// A map in order to convert block references into their corresponding functions
    pub block_to_function: HashMap<BlockReference, FunctionReference>,

    pub block_names: HashSet<String>,

    /// The functions contained in the module
    pub functions: Vec<Function>,

    /// A name -> index hashmap for function names
    pub function_names: HashMap<String, usize>,

    /// The current block of the module. Is used for the [`InstructionWriter`][`crate::writer::InstructionWriter`] trait
    pub pos_block: Option<BlockReference>,
    /// The current function of the module. Is used for the [`InstructionWriter`][`crate::writer::InstructionWriter`] trait
    pub pos_function: Option<FunctionReference>,
    /// Should instructions be added at the start of the block or at the end? Is used for the [`InstructionWriter`][`crate::writer::InstructionWriter`] trait
    pub pos_is_start: bool,

    /// The variable sync point. Is used for the [`VariableSynchronizer`][`crate::block::sync::VariableSynchronizer`] trait
    pub(crate) variable_sync_point: Option<BlockReference>,
}

impl Module {
    /// Creates a new [`Module`] with the given name
    pub fn new(name: String) -> Self {
        Self {
            name,
            blocks: vec![],
            block_to_function: HashMap::new(),
            block_names: HashSet::new(),

            functions: vec![],
            function_names: HashMap::new(),

            pos_block: None,
            pos_function: None,
            pos_is_start: false,

            variable_sync_point: None,
        }
    }

    /// Obtains an index for the creation of a new [`BaseSSAValue`][`crate::values::BaseSSAValue`].
    pub fn obtain_value_ind(&mut self, block: BlockReference) -> usize {
        self.functions[self.block_to_function[&block].id].obtain_value_ind()
    }

    /// Creates a [`Block`] with the given name for the given function.
    pub fn create_block(&mut self, name: String) -> RemirResult<BlockReference> {
        if self.pos_function.is_none() {
            return_err!("The current function is null! use Module::move_function first");
        }

        let reference = BlockReference::new(
            self.find_block_name(name.clone(), name, 0),
            self.blocks.len(),
        );

        let mut block = Block::new(reference.clone());

        self.inherit_sync_point(&mut block);

        let func_ref = self.pos_function.clone().unwrap();

        self.functions[func_ref.id].blocks.push(reference.clone());

        self.block_to_function.insert(reference.clone(), func_ref);

        self.blocks.push(block);
        Ok(reference)
    }

    fn find_block_name(&self, name: String, original: String, ind: usize) -> String {
        if !self.block_names.contains(&name) {
            name
        } else {
            self.find_block_name(format!("{}_{}", original, ind + 1), original, ind + 1)
        }
    }

    /// Creates a new function with the given name, arguments and return type inside of the module
    pub fn create_function(
        &mut self,
        name: String,
        arguments: Vec<ValueType>,
        return_type: Option<ValueType>,
    ) -> FunctionReference {
        let reference = FunctionReference::new(name, self.functions.len());

        let function = Function::new(reference.clone(), arguments, return_type);

        self.function_names
            .insert(reference.name.clone(), reference.id);
        self.functions.push(function);
        reference
    }

    /// Gets the function reference by name if it is currently registered inside of the module
    pub fn get_function_by_name(&self, name: String) -> Option<FunctionReference> {
        if !self.function_names.contains_key(&name) {
            return None;
        }

        Some(FunctionReference::new(
            name.clone(),
            self.function_names[&name],
        ))
    }

    /// Moves the current function indicator to the given reference.
    pub fn move_function(&mut self, func: FunctionReference) {
        self.pos_function = Some(func);
    }
}
