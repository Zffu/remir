use crate::{
    insts::Instruction,
    module::Module,
    utils::operators::CompareOperator,
    values::{float::SSAFloatValue, int::SSAIntValue},
    writer::InstructionWriter,
};

pub fn build_int_compare(
    module: &mut Module,
    a: SSAIntValue,
    b: SSAIntValue,
    op: CompareOperator,
    signed: bool,
) -> Result<SSAIntValue, ()> {
    if (a.signed != b.signed && !signed) || (a.signed && !signed) {
        return Err(()); // Enforces sign	
    }

    let inst = Instruction::CompareOperationInt { a, b, op, signed };

    let val = module.write(inst).get()?;

    val.try_into()
}

pub fn build_float_compare(
    module: &mut Module,
    a: SSAFloatValue,
    b: SSAFloatValue,
    op: CompareOperator,
    signed: bool,
) -> Result<SSAFloatValue, ()> {
    if (a.signed != b.signed && !signed) || (a.signed && !signed) {
        return Err(()); // Enforces sign	
    }

    let inst = Instruction::CompareOperationFloat { a, b, op, signed };

    let val = module.write(inst).get()?;

    val.try_into()
}
