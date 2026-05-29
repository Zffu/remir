use crate::{
    block::{self, BlockInstruction, BlockReference},
    insts::Instruction,
    module::Module,
};

/// Represents something that can write instructions
pub trait InstructionWriter {
    /// Moves at the start of the given block.
    fn move_start(&mut self, block: BlockReference);

    /// Moves at the end of the given block.
    fn move_end(&mut self, block: BlockReference);

    /// Writes an instruction at the given position
    fn write(&mut self, inst: Instruction) -> BlockInstruction;
}

impl InstructionWriter for Module {
    fn move_start(&mut self, block: BlockReference) {
        self.pos_block = Some(block);
        self.pos_is_start = true;
    }

    fn move_end(&mut self, block: BlockReference) {
        self.pos_block = Some(block);
        self.pos_is_start = false;
    }

    fn write(&mut self, inst: Instruction) -> BlockInstruction {
        if self.pos_block.is_none() {
            panic!("No block position defined! Use move_start or move_end before write");
        }

        todo!()
    }
}
