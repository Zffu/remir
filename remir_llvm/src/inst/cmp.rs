use std::hint::unreachable_unchecked;

use inkwell::{FloatPredicate, IntPredicate};
use remir::{block::BlockInstruction, insts::Instruction, utils::operators::CompareOperator};

use crate::{LLVMBridge, llvm_to_base, utils::LLVMBasicValue};

pub fn bridge_llvm_cmp_instruction(
    instruction: BlockInstruction,
    bridge: &mut LLVMBridge,
) -> Result<Option<LLVMBasicValue>, ()> {
    let res = match &instruction.instruction {
        Instruction::CompareOperationInt { a, b, op, signed } => {
            let a = bridge.values[&a.base.inst_ind].into_int_value();
            let b = bridge.values[&b.base.inst_ind].into_int_value();

            let predicate = match op {
                CompareOperator::Eq => IntPredicate::EQ,
                CompareOperator::Ne => IntPredicate::NE,
                CompareOperator::Lt => {
                    if *signed {
                        IntPredicate::SLT
                    } else {
                        IntPredicate::ULT
                    }
                }
                CompareOperator::Le => {
                    if *signed {
                        IntPredicate::SLE
                    } else {
                        IntPredicate::ULE
                    }
                }

                CompareOperator::Gt => {
                    if *signed {
                        IntPredicate::SGT
                    } else {
                        IntPredicate::UGT
                    }
                }

                CompareOperator::Ge => {
                    if *signed {
                        IntPredicate::SGE
                    } else {
                        IntPredicate::UGE
                    }
                }
            };

            let res = llvm_to_base!(bridge.builder.build_int_compare(predicate, a, b, ""));

            Some(res.into())
        }

        Instruction::CompareOperationFloat {
            a,
            b,
            op,
            signed: _,
        } => {
            let a = bridge.values[&a.base.inst_ind].into_float_value();
            let b = bridge.values[&b.base.inst_ind].into_float_value();

            let predicate = match op {
                CompareOperator::Eq => FloatPredicate::OEQ,
                CompareOperator::Ne => FloatPredicate::ONE,
                CompareOperator::Lt => FloatPredicate::OLT,
                CompareOperator::Le => FloatPredicate::OLE,
                CompareOperator::Gt => FloatPredicate::OGT,
                CompareOperator::Ge => FloatPredicate::OGE,
            };

            let res = llvm_to_base!(bridge.builder.build_float_compare(predicate, a, b, ""));

            Some(res.into())
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}
