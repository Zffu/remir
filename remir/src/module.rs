use crate::block::{Block, BlockReference};

pub struct Module {
    pub name: String,

    pub blocks: Vec<Block>,

    pub pos_block: Option<BlockReference>,
    pub pos_is_start: bool,

    value_index_counter: usize,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            blocks: vec![],

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
}
