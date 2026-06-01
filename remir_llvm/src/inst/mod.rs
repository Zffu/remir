use remir::{block::BlockInstruction, insts::Instruction, module::Module};

use crate::{
    LLVMBridge,
    inst::{
        branches::bridge_llvm_branch_instruction, cmp::bridge_llvm_cmp_instruction,
        consts::bridge_llvm_const_instruction, math::bridge_llvm_math_instruction,
        regs::bridge_llvm_reg_instructions,
    },
    utils::LLVMBasicValue,
};

pub mod branches;
pub mod cmp;
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

        Instruction::MathOperationInt { .. } | Instruction::MathOperationFloat { .. } => {
            bridge_llvm_math_instruction(instruction, bridge)
        }

        Instruction::CompareOperationInt { .. } | Instruction::CompareOperationFloat { .. } => {
            bridge_llvm_cmp_instruction(instruction, bridge)
        }

        Instruction::UncondBr { .. }
        | Instruction::Condbr { .. }
        | Instruction::IndirectBranch { .. }
        | Instruction::Phi { .. } => bridge_llvm_branch_instruction(instruction, bridge),

        _ => todo!(),
    }
}
