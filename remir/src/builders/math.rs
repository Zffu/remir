use crate::{
    insts::Instruction,
    misc::MathOperator,
    module::Module,
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
) -> Result<SSAIntValue, ()> {
    if (a.signed != b.signed && !signed) || (a.signed && !signed) {
        return Err(()); // Enforces sign	
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
) -> Result<SSAFloatValue, ()> {
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
