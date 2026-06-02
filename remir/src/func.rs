//! Declarations related to functions

use crate::{
    block::{Block, BlockReference, vars::BlockVariable},
    module::Module,
    values::{BaseSSAValue, ValueType},
};

/// Represents a reference to a [`Function`]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FunctionReference {
    /// The name of the function
    pub name: String,

    /// The internal ID (index) of the function
    pub id: usize,
}

/// Represents a function inside of the Remir IR.
pub struct Function {
    /// The self reference to the function
    pub reference: FunctionReference,

    /// The blocks owned by the function.
    pub blocks: Vec<BlockReference>,

    /// The argument types of the function
    pub arguments: Vec<(String, ValueType)>,

    /// The return type of the function
    pub return_type: Option<ValueType>,

    /// The counter used to generate [`BaseSSAValue`] indexes
    pub value_index_counter: usize,
}

impl Function {
    /// Creates a new [`Function`] with the given reference, argument types and return types
    pub fn new(
        reference: FunctionReference,
        arguments: Vec<(String, ValueType)>,
        return_type: Option<ValueType>,
    ) -> Self {
        Self {
            reference,
            blocks: vec![],
            arguments,
            return_type,
            value_index_counter: 0,
        }
    }

    /// Appends an entry block inside of the function
    ///
    /// # Panics
    /// This function will panic if the entry block is already present
    /// or if there are already blocks inside of the function
    ///
    #[deprecated(note = "will be replaced by Function::append_block")]
    pub fn append_entry_block(&mut self, module: &mut Module) -> BlockReference {
        if !self.blocks.is_empty() {
            panic!("Tried using append_entry_block on a non empty function");
        }

        let reference = module.create_block(
            format!("{}::entry", self.reference.name),
            self.reference.clone(),
        );

        let block = &mut module.blocks[reference.id];

        self.append_arguments(block);

        reference
    }

    fn append_arguments(&mut self, block: &mut Block) {
        let mut ind = 0;

        for arg in &self.arguments {
            let value = BaseSSAValue::new(ind, arg.1.clone());

            block.variables.insert(
                arg.0.clone(),
                BlockVariable::new_ssa(arg.0.clone(), Some(value)),
            );
            ind += 1;
        }
    }

    /// Appends a block inside of the [`Function`] with the given name and returns it's reference
    ///
    /// The block will be marked as owned by the function
    ///
    pub fn append_block(&mut self, module: &mut Module, name: String) -> BlockReference {
        let reference = module.create_block(
            format!("{}::{}", self.reference.name, name),
            self.reference.clone(),
        );

        self.blocks.push(reference.clone());

        reference
    }

    /// Obtains an index for the creation of a new [`BaseSSAValue`][`crate::values::BaseSSAValue`].
    pub fn obtain_value_ind(&mut self) -> usize {
        self.value_index_counter += 1;
        self.value_index_counter - 1
    }
}

impl FunctionReference {
    /// Creates a new [`FunctionReference`]
    pub fn new(name: String, id: usize) -> Self {
        Self { name, id }
    }
}
