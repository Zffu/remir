use std::collections::HashMap;

use crate::{
    block::{Block, BlockReference},
    func::{Function, FunctionReference},
    values::ValueType,
};

pub struct Module {
    pub name: String,

    pub blocks: Vec<Block>,
    pub block_to_function: HashMap<BlockReference, FunctionReference>,

    pub functions: Vec<Function>,

    pub pos_block: Option<BlockReference>,
    pub pos_is_start: bool,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            blocks: vec![],
            block_to_function: HashMap::new(),

            functions: vec![],

            pos_block: None,
            pos_is_start: false,
        }
    }

    /// Obtains an index for the creation of a new [`BaseSSAValue`][`crate::values::BaseSSAValue`].
    pub fn obtain_value_ind(&mut self, block: BlockReference) -> usize {
        self.functions[self.block_to_function[&block].id].obtain_value_ind()
    }

    pub fn create_block(&mut self, name: String, func: FunctionReference) -> BlockReference {
        let reference = BlockReference::new(name, self.blocks.len());

        let block = Block::new(reference.clone());

        self.block_to_function.insert(reference.clone(), func);

        self.blocks.push(block);
        reference
    }

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
}
