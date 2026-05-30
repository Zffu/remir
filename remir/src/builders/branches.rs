use crate::{
    block::BlockReference,
    insts::Instruction,
    module::Module,
    values::{BaseSSAValue, int::SSAIntValue, ptr::SSAPointerValue},
    writer::InstructionWriter,
};

pub fn build_unconditional_branch(module: &mut Module, branch: BlockReference) -> Result<(), ()> {
    let inst = Instruction::UncondBr { branch };

    module.write(inst);
    Ok(())
}

pub fn build_conditional_branch(
    module: &mut Module,
    cond: SSAIntValue,
    true_branch: BlockReference,
    false_branch: BlockReference,
) -> Result<(), ()> {
    cond.enforces_boolean()?;

    let inst = Instruction::Condbr {
        cond,
        true_label: true_branch,
        false_label: false_branch,
    };

    module.write(inst);
    Ok(())
}

pub fn build_indirect_branch(module: &mut Module, target: SSAPointerValue) -> Result<(), ()> {
    let inst = Instruction::IndirectBranch { target };

    module.write(inst);
    Ok(())
}

pub fn build_phi(
    module: &mut Module,
    label_set: Vec<(BlockReference, BaseSSAValue)>,
) -> Result<BaseSSAValue, ()> {
    let ty = label_set[0].1.value_type.clone();

    for entry in &label_set {
        if entry.1.value_type != ty {
            return Err(());
        }
    }

    let inst = Instruction::Phi { label_set };

    module.write(inst).get()
}
