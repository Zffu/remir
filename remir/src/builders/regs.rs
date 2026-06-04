use crate::{
    errs::RemirResult,
    insts::Instruction,
    module::Module,
    values::{BaseSSAValue, ptr::SSAPointerValue},
    writer::InstructionWriter,
};

pub fn build_copy(module: &mut Module, val: BaseSSAValue) -> RemirResult<BaseSSAValue> {
    let inst = Instruction::Copy { val };

    module.write(inst).get()
}

pub fn build_load(module: &mut Module, source: SSAPointerValue) -> RemirResult<BaseSSAValue> {
    let inst = Instruction::Load { source };

    module.write(inst).get()
}

pub fn build_store(
    module: &mut Module,
    destination: SSAPointerValue,
    source: BaseSSAValue,
) -> Result<(), ()> {
    if destination.inner_type != source.value_type {
        return Err(());
    }

    let inst = Instruction::Store {
        destination,
        source,
    };

    module.write(inst);
    Ok(())
}
