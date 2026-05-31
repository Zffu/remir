use crate::{
    insts::Instruction, module::Module, values::int::SSAIntValue, writer::InstructionWriter,
};

pub fn build_unreachable(module: &mut Module) -> Result<(), ()> {
    module.write(Instruction::Unreachable);
    Ok(())
}

pub fn build_crash(module: &mut Module, message: Option<String>) -> Result<(), ()> {
    let inst = Instruction::Crash { message };

    module.write(inst);
    Ok(())
}

pub fn build_assume(module: &mut Module, val: SSAIntValue) -> Result<(), ()> {
    val.enforces_boolean()?;

    let inst = Instruction::Assume { val };
    module.write(inst);

    Ok(())
}
