use std::hint::unreachable_unchecked;

use remir::{block::BlockInstruction, insts::Instruction};

use crate::{LLVMBridge, llvm_to_base, llvm_to_base_returnless, utils::LLVMBasicValue};

pub fn bridge_llvm_struct_instruction(
    instruction: BlockInstruction,
    bridge: &mut LLVMBridge,
) -> Result<Option<LLVMBasicValue>, ()> {
    let res = match &instruction.instruction {
        Instruction::ExtractValue { struct_val, index } => {
            let struct_val = bridge.values[&struct_val.base.inst_ind].into_struct_value();

            let res = llvm_to_base!(bridge.builder.build_extract_value(
                struct_val,
                *index as u32,
                ""
            ));

            Some(res.into())
        }

        Instruction::InsertValue {
            struct_val,
            index,
            val,
        } => {
            let struct_val = bridge.values[&struct_val.base.inst_ind].into_struct_value();
            let val = bridge.values[&val.inst_ind].inner;

            llvm_to_base_returnless!(bridge.builder.build_insert_value(
                struct_val,
                val,
                *index as u32,
                ""
            ));

            None
        }

        Instruction::Switch {
            value,
            else_block,
            cases,
        } => {
            let value = bridge.values[&value.base.inst_ind].into_int_value();
            let else_block = bridge.blocks[else_block].inner.clone();

            let mut llvm_cases = vec![];

            for case in cases {
                let case_val = bridge.values[&case.0.base.inst_ind].into_int_value();
                let block = bridge.blocks[&case.1].inner.clone();

                llvm_cases.push((case_val, block));
            }

            llvm_to_base_returnless!(bridge.builder.build_switch(value, else_block, &llvm_cases));

            None
        }

        Instruction::Select {
            cond,
            true_val,
            false_val,
        } => {
            let cond = bridge.values[&cond.base.inst_ind].into_int_value();
            let true_val = bridge.values[&true_val.inst_ind].inner.clone();
            let false_val = bridge.values[&false_val.inst_ind].inner.clone();

            let res = llvm_to_base!(bridge.builder.build_select(cond, true_val, false_val, ""));

            Some(res)
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}
