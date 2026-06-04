use crate::{
    errs::RemirResult,
    insts::Instruction,
    misc::MathOperator,
    module::Module,
    return_err,
    values::{float::SSAFloatValue, int::SSAIntValue},
    writer::InstructionWriter,
};

pub fn build_math_op_int(
    module: &mut Module,
    a: SSAIntValue,
    b: SSAIntValue,
    op: MathOperator,
    signed: bool,
    signed_wrap: bool,
    unsigned_wrap: bool,
    fast: bool,
) -> RemirResult<SSAIntValue> {
    if (a.signed != b.signed && !signed) || (a.signed && !signed) {
        return_err!("The sign of the math op and the signed states of the values are not the same"); // Enforces sign	
    }

    let inst = Instruction::MathOperationInt {
        a,
        b,
        op,
        signed,
        signed_wrap,
        unsigned_wrap,
        fast,
    };

    let val = module.write(inst).get()?;

    val.try_into()
}

pub fn build_math_op_float(
    module: &mut Module,
    a: SSAFloatValue,
    b: SSAFloatValue,
    op: MathOperator,
    signed_wrap: bool,
    unsigned_wrap: bool,
    fast: bool,
) -> RemirResult<SSAFloatValue> {
    let inst = Instruction::MathOperationFloat {
        a,
        b,
        op,
        signed_wrap,
        unsigned_wrap,
        fast,
    };

    let val = module.write(inst).get()?;

    val.try_into()
}
