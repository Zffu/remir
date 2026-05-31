use inkwell::values::BasicValueEnum;
use remir::{block::BlockInstruction, insts::Instruction, module::Module};

use crate::{LLVMBridge, inst::consts::bridge_llvm_const_instruction, utils::LLVMBasicValue};

pub mod consts;

pub fn bridge_llvm_instruction(instruction: BlockInstruction, func: usize, bridge: &mut LLVMBridge, module: &mut Module) -> Result<Option<LLVMBasicValue>, ()> {
	let res: Option<BasicValueEnum<'static>>  = match Instruction::from(instruction.clone().into()) {
		Instruction::ConstInt { .. } | Instruction::ConstFloat { ..} | Instruction::ConstPointer { .. } => return bridge_llvm_const_instruction(instruction, bridge)
	}
}