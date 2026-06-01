use remir::{block::BlockInstruction, insts::Instruction, module::Module};

use crate::{
    LLVMBridge,
    inst::{
        branches::bridge_llvm_branch_instruction, cmp::bridge_llvm_cmp_instruction,
        consts::bridge_llvm_const_instruction, funcs::bridge_llvm_function_instruction,
        math::bridge_llvm_math_instruction, mem::bridge_llvm_mem_instruction,
        regs::bridge_llvm_reg_instructions,
    },
    utils::LLVMBasicValue,
};

pub mod branches;
pub mod cmp;
pub mod consts;
pub mod funcs;
pub mod math;
pub mod mem;
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

        Instruction::Call { .. } | Instruction::RetNull | Instruction::Ret { .. } => {
            bridge_llvm_function_instruction(instruction, func, bridge, module)
        }

        Instruction::Alloc { .. }
        | Instruction::AllocUntyped { .. }
        | Instruction::Alloca { .. }
        | Instruction::AllocaUntyped { .. }
        | Instruction::Free { .. }
        | Instruction::Gep { .. }
        | Instruction::LoadIndexed { .. }
        | Instruction::StoreIndexed { .. } => {
            bridge_llvm_mem_instruction(instruction, func, bridge, module)
        }

        _ => todo!(),
    }
}
