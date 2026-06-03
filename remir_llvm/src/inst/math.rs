use std::hint::unreachable_unchecked;

use inkwell::values::{BasicValue, BasicValueEnum, FastMathFlags};
use remir::{block::BlockInstruction, insts::Instruction, misc::MathOperator};

use crate::{LLVMBridge, llvm_to_base, llvm_to_base_returnless, utils::LLVMBasicValue};

pub fn bridge_llvm_math_instruction(
    instruction: BlockInstruction,
    bridge: &mut LLVMBridge,
) -> Result<Option<LLVMBasicValue>, ()> {
    let res: Option<BasicValueEnum<'static>> = match &instruction.instruction {
        Instruction::MathOperationInt {
            a,
            b,
            op,
            signed,
            signed_wrap,
            unsigned_wrap,
            fast: _,
        } => {
            let a = bridge.values[&a.base.inst_ind].into_int_value();
            let b = bridge.values[&b.base.inst_ind].into_int_value();

            let res = match op {
                MathOperator::Add => bridge.builder.build_int_add(a, b, ""),
                MathOperator::Sub => bridge.builder.build_int_sub(a, b, ""),
                MathOperator::Mul => bridge.builder.build_int_mul(a, b, ""),
                MathOperator::Div => {
                    if *signed {
                        bridge.builder.build_int_signed_div(a, b, "")
                    } else {
                        bridge.builder.build_int_unsigned_div(a, b, "")
                    }
                }

                MathOperator::Mod => {
                    if *signed {
                        bridge.builder.build_int_signed_rem(a, b, "")
                    } else {
                        bridge.builder.build_int_unsigned_rem(a, b, "")
                    }
                }

                MathOperator::And => bridge.builder.build_and(a, b, ""),
                MathOperator::Or => bridge.builder.build_or(a, b, ""),
                MathOperator::Xor => bridge.builder.build_xor(a, b, ""),
                MathOperator::Shl => bridge.builder.build_left_shift(a, b, ""),
                MathOperator::Shr => bridge.builder.build_right_shift(a, b, *signed, ""),
            };

            let res = llvm_to_base!(res);

            if let Some(res2) = res.as_instruction_value() {
                llvm_to_base_returnless!(res2.set_no_signed_wrap_flag(!*signed_wrap));
                llvm_to_base_returnless!(res2.set_no_unsigned_wrap_flag(!*unsigned_wrap));
            }

            Some(res.into())
        }

        Instruction::MathOperationFloat {
            a,
            b,
            op,
            signed_wrap,
            unsigned_wrap,
            fast,
        } => {
            let a = bridge.values[&a.base.inst_ind].into_float_value();
            let b = bridge.values[&b.base.inst_ind].into_float_value();

            let res = match op {
                MathOperator::Add => bridge.builder.build_float_add(a, b, ""),
                MathOperator::Sub => bridge.builder.build_float_sub(a, b, ""),
                MathOperator::Mul => bridge.builder.build_float_mul(a, b, ""),
                MathOperator::Div => bridge.builder.build_float_div(a, b, ""),
                MathOperator::Mod => bridge.builder.build_float_rem(a, b, ""),

                _ => panic!(),
            };

            let res = llvm_to_base!(res);
            let res2 = res.as_instruction().unwrap();

            llvm_to_base_returnless!(res2.set_no_signed_wrap_flag(!*signed_wrap));
            llvm_to_base_returnless!(res2.set_no_unsigned_wrap_flag(!*unsigned_wrap));

            let mut flags = FastMathFlags::empty();

            if *fast {
                flags = FastMathFlags::all();
            }

            llvm_to_base_returnless!(res2.set_fast_math_flags(flags));

            Some(res.into())
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}
