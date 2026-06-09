use std::hint::unreachable_unchecked;

use remir::{block::BlockInstruction, insts::Instruction};

use crate::{LLVMBridge, llvm_to_base, llvm_to_base_returnless, utils::LLVMBasicValue};

pub fn bridge_llvm_branch_instruction(
    instruction: BlockInstruction,
    bridge: &mut LLVMBridge,
) -> Result<Option<LLVMBasicValue>, ()> {
    let res = match &instruction.instruction {
        Instruction::UncondBr { branch } => {
            let block = &bridge.blocks[branch];

            llvm_to_base_returnless!(
                bridge
                    .builder
                    .build_unconditional_branch(block.inner.clone())
            );

            None
        }

        Instruction::Condbr {
            cond,
            true_label,
            false_label,
        } => {
            let cond = bridge.values[&cond.base.inst_ind].into_int_value();

            let true_label = bridge.blocks[true_label].inner.clone();
            let false_label = bridge.blocks[false_label].inner.clone();

            llvm_to_base_returnless!(bridge.builder.build_conditional_branch(
                cond,
                true_label,
                false_label
            ));

            None
        }

        Instruction::IndirectBranch {
            target,
            destinations,
        } => {
            let target = bridge.values[&target.base.inst_ind].into_pointer_value();
            let mut dests = vec![];

            for destination in destinations {
                dests.push(bridge.blocks[destination].inner.clone());
            }

            llvm_to_base_returnless!(bridge.builder.build_indirect_branch(target, &dests));
            None
        }

        Instruction::Phi { label_set } => {
            let ty = bridge
                .type_storage
                .convert(label_set[0].1.value_type.clone());

            let phi = llvm_to_base!(bridge.builder.build_phi(ty.inner, ""));

            for choice in label_set {
                println!("Test: {}", choice.1.inst_ind);

                let block = bridge.blocks[&choice.0].clone();
                let value = bridge.values[&choice.1.inst_ind].clone();

                phi.add_incoming(&[(&value.inner, block.inner)]);
            }

            Some(phi.as_basic_value())
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}
