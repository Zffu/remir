use std::hint::unreachable_unchecked;

use inkwell::{
    attributes::{Attribute, AttributeLoc},
    llvm_sys::LLVMCallConv,
};
use remir::{block::BlockInstruction, insts::Instruction, module::Module};

use crate::{LLVMBridge, llvm_to_base, llvm_to_base_returnless, utils::LLVMBasicValue};

pub fn bridge_llvm_function_instruction(
    instruction: BlockInstruction,
    func: usize,
    bridge: &mut LLVMBridge,
    module: &mut Module,
) -> Result<Option<LLVMBasicValue>, ()> {
    let res = match &instruction.instruction {
        Instruction::Call {
            func_label,
            args,
            pure,
            no_return,
            fast_calling_conv,
        } => {
            let mut attributes = vec![];

            if *pure {
                attributes.push(
                    bridge
                        .ctx
                        .create_enum_attribute(Attribute::get_named_enum_kind_id("readnone"), 0),
                ); // Doesn't read memory

                attributes.push(
                    bridge
                        .ctx
                        .create_enum_attribute(Attribute::get_named_enum_kind_id("readonly"), 0),
                ); // Only reads memory on read access
            }

            if *no_return {
                attributes.push(
                    bridge
                        .ctx
                        .create_enum_attribute(Attribute::get_named_enum_kind_id("noreturn"), 0),
                );
            }

            let mut arguments = vec![];
            let func = bridge.functions[func_label].inner.clone();

            for arg in args {
                arguments.push(bridge.values[&arg.inst_ind].inner.into());
            }

            let res = llvm_to_base!(bridge.builder.build_call(func, &arguments, ""));

            for attribute in attributes {
                res.add_attribute(AttributeLoc::Function, attribute);
            }

            if *fast_calling_conv {
                res.set_call_convention(LLVMCallConv::LLVMFastCallConv as u32);
            }

            res.try_as_basic_value().basic()
        }

        Instruction::RetNull => {
            llvm_to_base_returnless!(bridge.builder.build_return(None));

            None
        }

        Instruction::Ret { val } => {
            let val = bridge.values[&val.inst_ind].inner.clone();

            llvm_to_base_returnless!(bridge.builder.build_return(Some(&val)));
            None
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}
