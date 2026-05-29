use crate::block::Block;

pub struct Module {
    pub name: String,

    pub blocks: Vec<Block>,

    value_index_counter: usize,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            blocks: vec![],
            value_index_counter: 0,
        }
    }

    /// Obtains an index for the creation of a new [`BaseSSAValue`][`crate::values::BaseSSAValue`].
    pub fn obtain_value_ind(&mut self) -> usize {
        self.value_index_counter += 1;
        self.value_index_counter - 1
    }
}
