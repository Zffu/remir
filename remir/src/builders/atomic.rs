use crate::{
    insts::Instruction,
    module::Module,
    utils::atomic::MemoryOrder,
    values::{BaseSSAValue, ptr::SSAPointerValue},
    writer::InstructionWriter,
};

pub fn build_load_atomic(
    module: &mut Module,
    source: SSAPointerValue,
    ordering: MemoryOrder,
) -> Result<BaseSSAValue, ()> {
    let inst = Instruction::LoadAtomic { source, ordering };

    module.write(inst).get()
}

pub fn build_store_atomic(
    module: &mut Module,
    dest: SSAPointerValue,
    val: BaseSSAValue,
    ordering: MemoryOrder,
) -> Result<(), ()> {
    let inst = Instruction::StoreAtomic {
        dest,
        val,
        ordering,
    };

    module.write(inst);
    Ok(())
}

pub fn build_fence(module: &mut Module, ordering: MemoryOrder) -> Result<(), ()> {
    let inst = Instruction::Fence { ordering };

    module.write(inst);
    Ok(())
}
