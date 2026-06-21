use std::hint::unreachable_unchecked;

use inkwell::values::BasicValueEnum;
use remir::{block::BlockInstruction, insts::Instruction, values::ValueType};

use crate::{LLVMBridge, llvm_to_base, utils::LLVMBasicValue};

pub fn bridge_llvm_vals_instruction(
    instruction: BlockInstruction,
    bridge: &mut LLVMBridge,
) -> Result<Option<LLVMBasicValue>, ()> {
    let res: Option<BasicValueEnum<'static>> = match &instruction.instruction {
        Instruction::IntToFloat { val, into } => {
            if let ValueType::Int(signed, _) = &val.base.value_type {
                let val = bridge.values[&val.base.inst_ind].into_int_value();

                let into = bridge.type_storage.convert(into.clone());

                let res;

                if *signed {
                    res = bridge
                        .builder
                        .build_signed_int_to_float(val, into.into_float_type(), "");
                } else {
                    res =
                        bridge
                            .builder
                            .build_unsigned_int_to_float(val, into.into_float_type(), "");
                }

                let res = llvm_to_base!(res);

                Some(res.into())
            } else {
                unsafe { unreachable_unchecked() }
            }
        }

        Instruction::FloatToInt { val, into } => {
            if let ValueType::Int(signed, _) = into {
                let val = bridge.values[&val.base.inst_ind].into_float_value();

                let into = bridge.type_storage.convert(into.clone());

                let res;

                if *signed {
                    res = bridge
                        .builder
                        .build_float_to_signed_int(val, into.into_int_type(), "");
                } else {
                    res = bridge
                        .builder
                        .build_float_to_unsigned_int(val, into.into_int_type(), "");
                }

                let res = llvm_to_base!(res);

                Some(res.into())
            } else {
                unsafe { unreachable_unchecked() }
            }
        }

        Instruction::IntExtend { val, into } => {
            let signed = val.signed;

            let val = bridge.values[&val.base.inst_ind].into_int_value();
            let into = bridge.type_storage.convert(into.clone());

            let res;

            if signed {
                res = bridge
                    .builder
                    .build_int_s_extend(val, into.into_int_type(), "");
            } else {
                res = bridge
                    .builder
                    .build_int_z_extend(val, into.into_int_type(), "");
            }

            let res = llvm_to_base!(res);

            Some(res.into())
        }

        Instruction::IntTruncate { val, into } => {
            let val = bridge.values[&val.base.inst_ind].into_int_value();
            let into = bridge.type_storage.convert(into.clone());

            let res = llvm_to_base!(bridge.builder.build_int_truncate(
                val,
                into.into_int_type(),
                ""
            ));

            Some(res.into())
        }

        Instruction::FloatExtend { val, into } => {
            let val = bridge.values[&val.base.inst_ind].into_float_value();
            let into = bridge.type_storage.convert(into.clone());

            let res = llvm_to_base!(bridge.builder.build_float_ext(
                val,
                into.into_float_type(),
                ""
            ));

            Some(res.into())
        }

        Instruction::FloatTruncate { val, into } => {
            let val = bridge.values[&val.base.inst_ind].into_float_value();
            let into = bridge.type_storage.convert(into.clone());

            let res = llvm_to_base!(bridge.builder.build_float_trunc(
                val,
                into.into_float_type(),
                ""
            ));

            Some(res.into())
        }

        Instruction::IntToPtr { val, into } => {
            let val = bridge.values[&val.base.inst_ind].into_int_value();
            let into = bridge.type_storage.convert(into.clone());

            let res = llvm_to_base!(bridge.builder.build_int_to_ptr(
                val,
                into.into_pointer_type(),
                ""
            ));

            Some(res.into())
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}
