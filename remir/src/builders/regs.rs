use crate::{
    errs::RemirResult,
    insts::Instruction,
    module::Module,
    return_err,
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
) -> RemirResult<()> {
    if destination.inner_type != source.value_type {
        return_err!("the destination type and source type aren't the same!");
    }

    let inst = Instruction::Store {
        destination,
        source,
    };

    module.write(inst);
    Ok(())
}
