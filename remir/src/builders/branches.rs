use crate::{
    block::BlockReference,
    errs::RemirResult,
    insts::Instruction,
    module::Module,
    return_err,
    values::{BaseSSAValue, int::SSAIntValue, ptr::SSAPointerValue},
    writer::InstructionWriter,
};

pub fn build_unconditional_branch(module: &mut Module, branch: BlockReference) {
    let origin = module.pos_block.clone().unwrap();

    module.blocks[branch.id].origins.insert(origin.clone()); // Append origin

    module.blocks[origin.id].destinations.insert(branch.clone()); // Append destination

    let inst = Instruction::UncondBr { branch };

    module.write(inst);
}

pub fn build_conditional_branch(
    module: &mut Module,
    cond: SSAIntValue,
    true_branch: BlockReference,
    false_branch: BlockReference,
) -> RemirResult<()> {
    cond.enforces_boolean()?;

    let origin = module.pos_block.clone().unwrap();

    // Origin

    module.blocks[true_branch.id].origins.insert(origin.clone());
    module.blocks[false_branch.id]
        .origins
        .insert(origin.clone());

    // Destination

    module.blocks[origin.id]
        .destinations
        .insert(true_branch.clone());
    module.blocks[origin.id]
        .destinations
        .insert(false_branch.clone());

    let inst = Instruction::Condbr {
        cond,
        true_label: true_branch,
        false_label: false_branch,
    };

    module.write(inst);
    Ok(())
}

pub fn build_indirect_branch(
    module: &mut Module,
    target: SSAPointerValue,
    destinations: Vec<BlockReference>,
) {
    let inst = Instruction::IndirectBranch {
        target,
        destinations,
    };

    module.write(inst);
}

pub fn build_phi(
    module: &mut Module,
    label_set: Vec<(BlockReference, BaseSSAValue)>,
) -> RemirResult<BaseSSAValue> {
    let ty = label_set[0].1.value_type.clone();

    for entry in &label_set {
        if entry.1.value_type != ty {
            return_err!("The label set values are not of the same type");
        }
    }

    let inst = Instruction::Phi { label_set };

    module.write(inst).get()
}
