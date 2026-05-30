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
        }
    }

    pub fn append_entry_block(&mut self, module: &mut Module) -> BlockReference {
        if !self.blocks.is_empty() {
            panic!("Tried using append_entry_block on a non empty function");
        }

        let reference = module.create_block(format!("{}::entry", self.reference.name));
        let block = &mut module.blocks[reference.id];

        todo!("Add argument parsing")
    }

    pub fn append_block(&mut self, module: &mut Module, name: String) -> BlockReference {
        let reference = module.create_block(format!("{}::{}", self.reference.name, name));

        self.blocks.push(reference.clone());

        reference
    }
}

impl FunctionReference {
    pub fn new(name: String, id: usize) -> Self {
        Self { name, id }
    }
}
