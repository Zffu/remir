use std::hint::unreachable_unchecked;

use inkwell::values::BasicValueEnum;
use remir::{block::BlockInstruction, insts::Instruction};

use crate::{LLVMBridge, llvm_to_base, llvm_to_base_returnless, utils::LLVMBasicValue};

pub fn bridge_llvm_reg_instructions(
    instruction: BlockInstruction,
    bridge: &mut LLVMBridge,
) -> Result<Option<LLVMBasicValue>, ()> {
    let res: Option<BasicValueEnum<'static>> = match &instruction.instruction {
        Instruction::Copy { val } => Some(bridge.values[&val.inst_ind].clone().innner),

        Instruction::Load { source } => {
            let llvm_ptr = bridge.values[&source.base.inst_ind].into_pointer_value();
            let inner_type = bridge.type_storage.convert(source.inner_type.clone());

            let val = llvm_to_base!(bridge.builder.build_load(*inner_type, llvm_ptr, ""));

            Some(val)
        }

        Instruction::Store {
            destination,
            source,
        } => {
            let llvm_destination = bridge.values[&destination.base.inst_ind].into_pointer_value();
            let llvm_source = bridge.values[&source.inst_ind].clone();

            llvm_to_base_returnless!(bridge.builder.build_store(llvm_destination, *llvm_source));
            None
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}
