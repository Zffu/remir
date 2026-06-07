use remir::{block::BlockInstruction, func::FunctionReference, insts::Instruction, module::Module};

use crate::{
    LLVMBridge,
    inst::{
        atomic::bridge_llvm_atomic_instruction, branches::bridge_llvm_branch_instruction,
        cmp::bridge_llvm_cmp_instruction, consts::bridge_llvm_const_instruction,
        funcs::bridge_llvm_function_instruction, hints::bridge_llvm_hints_instruction,
        math::bridge_llvm_math_instruction, mem::bridge_llvm_mem_instruction,
        regs::bridge_llvm_reg_instructions, structs::bridge_llvm_struct_instruction,
        vals::bridge_llvm_vals_instruction,
    },
    utils::LLVMBasicValue,
};

pub mod atomic;
pub mod branches;
pub mod cmp;
pub mod consts;
pub mod funcs;
pub mod hints;
pub mod math;
pub mod mem;
pub mod regs;
pub mod structs;
pub mod vals;

pub fn bridge_llvm_instruction(
    instruction: BlockInstruction,
    bridge: &mut LLVMBridge,
    func: FunctionReference,
    module: &mut Module,
) -> Result<Option<LLVMBasicValue>, ()> {
    match &instruction.instruction {
        Instruction::ConstInt { .. }
        | Instruction::ConstFloat { .. }
        | Instruction::ConstPointer { .. }
        | Instruction::ConstString { .. }
        | Instruction::ConstStruct { .. } => {
            bridge_llvm_const_instruction(instruction, bridge, module)
        }
        Instruction::Copy { .. } | Instruction::Load { .. } | Instruction::Store { .. } => {
            bridge_llvm_reg_instructions(instruction, bridge)
        }

        Instruction::Not { .. }
        | Instruction::MathOperationInt { .. }
        | Instruction::MathOperationFloat { .. } => {
            bridge_llvm_math_instruction(instruction, bridge)
        }

        Instruction::CompareOperationInt { .. } | Instruction::CompareOperationFloat { .. } => {
            bridge_llvm_cmp_instruction(instruction, bridge)
        }

        Instruction::UncondBr { .. }
        | Instruction::Condbr { .. }
        | Instruction::IndirectBranch { .. }
        | Instruction::Phi { .. } => bridge_llvm_branch_instruction(instruction, bridge),

        Instruction::Call { .. }
        | Instruction::RetNull
        | Instruction::Ret { .. }
        | Instruction::GrabArgument { .. } => {
            bridge_llvm_function_instruction(instruction, func, bridge)
        }

        Instruction::Alloc { .. }
        | Instruction::AllocUntyped { .. }
        | Instruction::Alloca { .. }
        | Instruction::AllocaUntyped { .. }
        | Instruction::Free { .. }
        | Instruction::Gep { .. }
        | Instruction::LoadIndexed { .. }
        | Instruction::StoreIndexed { .. } => {
            bridge_llvm_mem_instruction(instruction, bridge, module)
        }

        Instruction::IntToFloat { .. }
        | Instruction::FloatToInt { .. }
        | Instruction::IntExtend { .. }
        | Instruction::IntTruncate { .. }
        | Instruction::FloatExtend { .. }
        | Instruction::FloatTruncate { .. } => bridge_llvm_vals_instruction(instruction, bridge),

        Instruction::Select { .. }
        | Instruction::ExtractValue { .. }
        | Instruction::InsertValue { .. }
        | Instruction::Switch { .. } => bridge_llvm_struct_instruction(instruction, bridge),

        Instruction::LoadAtomic { .. } | Instruction::StoreAtomic { .. } => {
            bridge_llvm_atomic_instruction(instruction, bridge)
        }

        Instruction::Assume { .. }
        | Instruction::Crash { .. }
        | Instruction::BitCast { .. }
        | Instruction::Unreachable => bridge_llvm_hints_instruction(instruction, bridge, module),
    }
}
