use remir::{block::BlockInstruction, insts::Instruction, module::Module};

use crate::{
    LLVMBridge,
    inst::{consts::bridge_llvm_const_instruction, regs::bridge_llvm_reg_instructions},
    utils::LLVMBasicValue,
};

pub mod consts;
pub mod math;
pub mod regs;

pub fn bridge_llvm_instruction(
    instruction: BlockInstruction,
    func: usize,
    bridge: &mut LLVMBridge,
    module: &mut Module,
) -> Result<Option<LLVMBasicValue>, ()> {
    match &instruction.instruction {
        Instruction::ConstInt { .. }
        | Instruction::ConstFloat { .. }
        | Instruction::ConstPointer { .. } => bridge_llvm_const_instruction(instruction, bridge),
        Instruction::Copy { .. } | Instruction::Load { .. } | Instruction::Store { .. } => {
            bridge_llvm_reg_instructions(instruction, bridge)
        }

        _ => todo!(),
    }
}
