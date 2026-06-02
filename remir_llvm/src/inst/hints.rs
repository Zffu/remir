use std::hint::unreachable_unchecked;

use inkwell::intrinsics::Intrinsic;
use remir::{block::BlockInstruction, insts::Instruction, module::Module};

use crate::{LLVMBridge, llvm_to_base, llvm_to_base_returnless, utils::LLVMBasicValue};

pub fn bridge_llvm_hints_instruction(
    instruction: BlockInstruction,
    bridge: &mut LLVMBridge,
    module: &mut Module,
) -> Result<Option<LLVMBasicValue>, ()> {
    let res = match &instruction.instruction {
        Instruction::Unreachable => {
            llvm_to_base_returnless!(bridge.builder.build_unreachable());

            None
        }

        Instruction::BitCast { src, into } => {
            let src = bridge.values[&src.inst_ind].inner.clone();
            let into = bridge.type_storage.convert(into.clone()).inner;

            let res = llvm_to_base!(bridge.builder.build_bit_cast(src, into, ""));

            Some(res)
        }

        Instruction::Crash { message: _ } => {
            let trap_fn = Intrinsic::find("llvm.trap")
                .unwrap()
                .get_declaration(&bridge.modules[&module.name], &[])
                .unwrap();

            llvm_to_base_returnless!(bridge.builder.build_call(trap_fn, &[], ""));
            llvm_to_base_returnless!(bridge.builder.build_unreachable());

            None
        }

        Instruction::Assume { val } => {
            let val = bridge.values[&val.base.inst_ind].into_int_value();

            let assume_fn = Intrinsic::find("llvm.assume")
                .unwrap()
                .get_declaration(&bridge.modules[&module.name], &[])
                .unwrap();

            llvm_to_base_returnless!(bridge.builder.build_call(assume_fn, &[val.into()], ""));

            None
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}
