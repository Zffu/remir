use crate::{block::BlockReference, module::Module, values::ValueType};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FunctionReference {
    pub name: String,
    pub id: usize,
}

pub struct Function {
    pub reference: FunctionReference,

    pub blocks: Vec<BlockReference>,

    pub arguments: Vec<ValueType>,
    pub return_type: Option<ValueType>,

    pub value_index_counter: usize,
}

impl Function {
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

    pub fn append_entry_block(&mut self, module: &mut Module) -> BlockReference {
        if !self.blocks.is_empty() {
            panic!("Tried using append_entry_block on a non empty function");
        }

        let reference = module.create_block(
            format!("{}::entry", self.reference.name),
            self.reference.clone(),
        );
        let block = &mut module.blocks[reference.id];

        todo!("Add argument parsing")
    }

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
    pub fn new(name: String, id: usize) -> Self {
        Self { name, id }
    }
}
