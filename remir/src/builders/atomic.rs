use crate::{
    errs::RemirResult,
    insts::Instruction,
    misc::MemoryOrder,
    module::Module,
    return_err,
    values::{BaseSSAValue, ptr::SSAPointerValue},
    writer::InstructionWriter,
};

pub fn build_load_atomic(
    module: &mut Module,
    source: SSAPointerValue,
    ordering: MemoryOrder,
) -> RemirResult<BaseSSAValue> {
    let inst = Instruction::LoadAtomic { source, ordering };

    module.write(inst).get()
}

pub fn build_store_atomic(
    module: &mut Module,
    dest: SSAPointerValue,
    val: BaseSSAValue,
    ordering: MemoryOrder,
) -> RemirResult<()> {
    if dest.inner_type != val.value_type {
        return_err!("The destination inner type is not the same as the value type");
    }

    let inst = Instruction::StoreAtomic {
        dest,
        val,
        ordering,
    };

    module.write(inst);
    Ok(())
}
