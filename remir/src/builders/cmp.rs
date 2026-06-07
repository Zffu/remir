use crate::{
    errs::RemirResult,
    insts::Instruction,
    misc::CompareOperator,
    module::Module,
    return_err,
    values::{float::SSAFloatValue, int::SSAIntValue},
    writer::InstructionWriter,
};

pub fn build_int_compare(
    module: &mut Module,
    a: SSAIntValue,
    b: SSAIntValue,
    op: CompareOperator,
    signed: bool,
) -> RemirResult<SSAIntValue> {
    if (a.signed != b.signed && !signed) || (a.signed && !signed) {
        return_err!("The sign of the compare and the signed states of the values are not the same"); // Enforces sign	
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
) -> RemirResult<SSAIntValue> {
    let inst = Instruction::CompareOperationFloat { a, b, op };

    let val = module.write(inst).get()?;

    val.try_into()
}
