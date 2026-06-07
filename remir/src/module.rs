use std::collections::HashMap;

use crate::{
    block::{Block, BlockReference},
    func::{Function, FunctionReference},
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

    /// The functions contained in the module
    pub functions: Vec<Function>,

    /// A name -> index hashmap for function names
    pub function_names: HashMap<String, usize>,

    pub pos_block: Option<BlockReference>,
    pub pos_function: Option<FunctionReference>,
    pub pos_is_start: bool,
}

impl Module {
    /// Creates a new [`Module`] with the given name
    pub fn new(name: String) -> Self {
        Self {
            name,
            blocks: vec![],
            block_to_function: HashMap::new(),

            functions: vec![],
            function_names: HashMap::new(),

            pos_block: None,
            pos_function: None,
            pos_is_start: false,
        }
    }

    /// Obtains an index for the creation of a new [`BaseSSAValue`][`crate::values::BaseSSAValue`].
    pub fn obtain_value_ind(&mut self, block: BlockReference) -> usize {
        self.functions[self.block_to_function[&block].id].obtain_value_ind()
    }

    /// Creates a [`Block`] with the given name for the given function.
    pub fn create_block(&mut self, name: String, func: FunctionReference) -> BlockReference {
        let reference = BlockReference::new(name, self.blocks.len());

        let block = Block::new(reference.clone());

        self.block_to_function.insert(reference.clone(), func);

        self.blocks.push(block);
        reference
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

    pub fn get_function(&mut self, r: &FunctionReference) -> &'static mut Function {
        unsafe {
            std::mem::transmute::<&mut Function, &'static mut Function>(&mut self.functions[r.id])
        }
    }
}
