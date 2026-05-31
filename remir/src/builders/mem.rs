use crate::{
    insts::Instruction,
    module::Module,
    values::{BaseSSAValue, ValueType, int::SSAIntValue, ptr::SSAPointerValue},
    writer::InstructionWriter,
};

pub fn build_alloc(
    module: &mut Module,
    size: SSAIntValue,
    val_type: Option<ValueType>,
) -> Result<SSAPointerValue, ()> {
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
) -> Result<SSAPointerValue, ()> {
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

pub fn build_free(module: &mut Module, ptr: SSAPointerValue) -> Result<(), ()> {
    let inst = Instruction::Free { ptr };

    module.write(inst);
    Ok(())
}

pub fn build_gep(
    module: &mut Module,
    base: SSAPointerValue,
    offset: SSAIntValue,
) -> Result<SSAPointerValue, ()> {
    let inst = Instruction::Gep { base, offset };

    let val = module.write(inst).get()?;

    val.try_into()
}

pub fn build_load_indexed(
    module: &mut Module,
    base: SSAPointerValue,
    index: SSAIntValue,
) -> Result<BaseSSAValue, ()> {
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
