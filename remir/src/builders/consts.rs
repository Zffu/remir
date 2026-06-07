use crate::{
    errs::{RemirError, RemirResult},
    insts::Instruction,
    module::Module,
    return_err,
    values::{
        BaseSSAValue, ValueType, array::SSAArrayValue, float::SSAFloatValue, int::SSAIntValue,
        ptr::SSAPointerValue, structs::SSAStructValue,
    },
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

pub fn build_const_struct(
    module: &mut Module,
    ty: ValueType,
    values: Vec<BaseSSAValue>,
) -> RemirResult<SSAStructValue> {
    if let ValueType::Struct(fields) = ty.clone() {
        for (i, val) in values.iter().enumerate() {
            if *fields[i] != val.value_type {
                return_err!("mismatched types for fields");
            }
        }

        let inst = Instruction::ConstStruct { ty, values };

        let val = module.write(inst).get()?;
        val.try_into()
    } else {
        return_err!("type is not struct")
    }
}

pub fn build_const_array(
    module: &mut Module,
    values: Vec<BaseSSAValue>,
) -> RemirResult<SSAArrayValue> {
    let ty = values[0].value_type.clone();

    for val in &values {
        if val.value_type != ty {
            return_err!("mismatched types");
        }
    }

    let inst = Instruction::ConstArray { values };
    let val = module.write(inst).get()?;

    val.try_into()
}

pub fn build_const_array_same(
    module: &mut Module,
    value: BaseSSAValue,
    count: usize,
) -> RemirResult<SSAArrayValue> {
    let inst = Instruction::ConstArraySame { value, count };
    let val = module.write(inst).get()?;

    val.try_into()
}
