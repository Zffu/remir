//! The lowering pass for lazy loads
//! Only use this if you used lazy load instructions

use crate::{errs::RemirResult, insts::Instruction, module::Module};

pub fn lazy_pass(module: &mut Module) -> RemirResult<()> {
    for block_index in 0..module.blocks.len() {
        let instructions = std::mem::take(&mut module.blocks[block_index].instructions);

        let mut new_instructions = vec![];

        for instruction in instructions {
            if let Instruction::LazyLoad {
                block,
                variable_name,
                ty: _,
            } = &instruction.instruction
            {
                let block_variable = module.blocks[block.id].variables[variable_name].clone();
                let val = block_variable.read(module)?;

                new_instructions.push(instruction.with_changed_inst(Instruction::Copy { val }))
            } else {
                new_instructions.push(instruction);
            }
        }

        module.blocks[block_index].instructions = new_instructions;
    }

    Ok(())
}
