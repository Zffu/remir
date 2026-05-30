use std::collections::HashMap;

use crate::{
    block::{Block, BlockReference},
    func::{self, Function, FunctionReference},
    values::ValueType,
};

pub struct Module {
    pub name: String,

    pub blocks: Vec<Block>,
    pub block_to_function: HashMap<BlockReference, FunctionReference>,

    pub functions: Vec<Function>,

    pub pos_block: Option<BlockReference>,
    pub pos_is_start: bool,

    value_index_counter: usize,
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

            value_index_counter: 0,
        }
    }

    /// Obtains an index for the creation of a new [`BaseSSAValue`][`crate::values::BaseSSAValue`].
    pub fn obtain_value_ind(&mut self) -> usize {
        self.value_index_counter += 1;
        self.value_index_counter - 1
    }

    pub fn create_block(&mut self, name: String) -> BlockReference {
        let reference = BlockReference::new(name, self.blocks.len());

        let block = Block::new(reference.clone());

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
