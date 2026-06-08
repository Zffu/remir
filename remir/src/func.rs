//! Declarations related to functions

use crate::{block::BlockReference, module::Module, values::ValueType};

/// Represents a reference to a [`Function`]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FunctionReference {
    /// The name of the function
    pub name: String,

    /// The internal ID (index) of the function
    pub id: usize,
}

/// Represents a function inside of the Remir IR.
#[derive(Clone)]
pub struct Function {
    /// The self reference to the function
    pub reference: FunctionReference,

    /// The blocks owned by the function.
    pub blocks: Vec<BlockReference>,

    /// The argument types of the function
    pub arguments: Vec<ValueType>,

    /// The return type of the function
    pub return_type: Option<ValueType>,

    /// The counter used to generate [`BaseSSAValue`][`crate::values::BaseSSAValue`] indexes
    pub value_index_counter: usize,
}

impl Function {
    /// Creates a new [`Function`] with the given reference, argument types and return types
    pub fn new(
        reference: FunctionReference,
        arguments: Vec<ValueType>,
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
