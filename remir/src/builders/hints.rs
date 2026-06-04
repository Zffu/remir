use crate::{
    errs::RemirResult, insts::Instruction, module::Module, values::int::SSAIntValue,
    writer::InstructionWriter,
};

pub fn build_unreachable(module: &mut Module) {
    module.write(Instruction::Unreachable);
}

pub fn build_crash(module: &mut Module, message: Option<String>) {
    let inst = Instruction::Crash { message };

    module.write(inst);
}

pub fn build_assume(module: &mut Module, val: SSAIntValue) -> RemirResult<()> {
    val.enforces_boolean()?;

    let inst = Instruction::Assume { val };
    module.write(inst);

    Ok(())
}
