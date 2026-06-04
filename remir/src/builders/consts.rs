use crate::{
    errs::RemirResult,
    insts::Instruction,
    module::Module,
    values::{float::SSAFloatValue, int::SSAIntValue, ptr::SSAPointerValue},
    writer::InstructionWriter,
};

pub fn build_const_int(
    module: &mut Module,
    val: i128,
    size: usize,
    signed: bool,
) -> RemirResult<SSAIntValue> {
    let inst = Instruction::ConstInt { val, size, signed };

    let val = module.write(inst).get()?;

    val.try_into()
}

pub fn build_const_float(module: &mut Module, val: f64, size: usize) -> RemirResult<SSAFloatValue> {
    let inst = Instruction::ConstFloat { val, size };

    let val = module.write(inst).get()?;

    val.try_into()
}

pub fn build_const_ptr(module: &mut Module, addr: usize) -> RemirResult<SSAPointerValue> {
    let inst = Instruction::ConstPointer { addr };

    let val = module.write(inst).get()?;

    val.try_into()
}

pub fn build_const_string(module: &mut Module, str: String) -> RemirResult<SSAPointerValue> {
    let inst = Instruction::ConstString { str };

    let val = module.write(inst).get()?;

    val.try_into()
}
