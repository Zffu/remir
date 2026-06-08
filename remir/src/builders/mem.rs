use crate::{
    errs::RemirResult,
    insts::Instruction,
    module::Module,
    return_err,
    values::{BaseSSAValue, ValueType, int::SSAIntValue, ptr::SSAPointerValue},
    writer::InstructionWriter,
};

pub fn build_alloc(
    module: &mut Module,
    size: SSAIntValue,
    val_type: Option<ValueType>,
) -> RemirResult<SSAPointerValue> {
    let inst;

    if val_type.is_none() {
        inst = Instruction::AllocUntyped { size };
    } else {
        inst = Instruction::Alloc {
            size,
            val_type: val_type.unwrap(),
        }
    }

    let val = module.write(inst).get()?;

    val.try_into()
}

pub fn build_alloca(
    module: &mut Module,
    size: SSAIntValue,
    val_type: Option<ValueType>,
) -> RemirResult<SSAPointerValue> {
    let inst;

    if val_type.is_none() {
        inst = Instruction::AllocaUntyped { size };
    } else {
        inst = Instruction::Alloca {
            size,
            val_type: val_type.unwrap(),
        }
    }

    let val = module.write(inst).get()?;

    val.try_into()
}

pub fn build_free(module: &mut Module, ptr: SSAPointerValue) {
    let inst = Instruction::Free { ptr };

    module.write(inst);
}

pub fn build_gep(
    module: &mut Module,
    base: SSAPointerValue,
    offset: SSAIntValue,
) -> RemirResult<SSAPointerValue> {
    let inst = Instruction::Gep { base, offset };

    let val = module.write(inst).get()?;

    val.try_into()
}

pub fn build_struct_gep(
    module: &mut Module,
    base: SSAPointerValue,
    field: usize,
) -> RemirResult<BaseSSAValue> {
    if let ValueType::Struct(fields) = &base.inner_type {
        if field >= fields.len() {
            return_err!("field >= fields.len");
        }

        let inst = Instruction::GepStruct { base, field };
        let val = module.write(inst).get()?;

        Ok(val)
    } else {
        return_err!("type is not a struct")
    }
}

pub fn build_load_indexed(
    module: &mut Module,
    base: SSAPointerValue,
    index: SSAIntValue,
) -> RemirResult<BaseSSAValue> {
    let inst = Instruction::LoadIndexed { base, index };

    module.write(inst).get()
}

pub fn build_load_store_indexed(
    module: &mut Module,
    base: SSAPointerValue,
    index: SSAIntValue,
    val: BaseSSAValue,
) -> Result<(), ()> {
    if base.inner_type != base.inner_type {
        return Err(()); // Cannot store a value that isn't the same as the pointer type
    }

    let inst = Instruction::StoreIndexed { base, index, val };

    module.write(inst);
    Ok(())
}
