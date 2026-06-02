use std::hint::unreachable_unchecked;

use inkwell::{AtomicOrdering, values::BasicValue};
use remir::{block::BlockInstruction, insts::Instruction, utils::atomic::MemoryOrder};

use crate::{LLVMBridge, llvm_to_base, llvm_to_base_returnless, utils::LLVMBasicValue};

pub fn bridge_llvm_atomic_instruction(
    instruction: BlockInstruction,
    bridge: &mut LLVMBridge,
) -> Result<Option<LLVMBasicValue>, ()> {
    let res = match &instruction.instruction {
        Instruction::LoadAtomic { source, ordering } => {
            let pointer_ty = bridge.type_storage.convert(source.inner_type.clone());
            let source = bridge.values[&source.base.inst_ind].into_pointer_value();

            let load = llvm_to_base!(bridge.builder.build_load(pointer_ty.inner, source, ""));
            let load2 = load.as_instruction_value().unwrap();

            llvm_to_base_returnless!(load2.set_atomic_ordering(convert_ordering(ordering)));

            Some(load)
        }

        Instruction::StoreAtomic {
            dest,
            val,
            ordering,
        } => {
            let source = bridge.values[&dest.base.inst_ind].into_pointer_value();
            let val = bridge.values[&val.inst_ind].inner.clone();

            let res = llvm_to_base!(bridge.builder.build_store(source, val));

            llvm_to_base_returnless!(res.set_atomic_ordering(convert_ordering(ordering)));

            None
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}

pub fn convert_ordering(ordering: &MemoryOrder) -> AtomicOrdering {
    match ordering {
        MemoryOrder::Relaxed => AtomicOrdering::Monotonic,
        MemoryOrder::Consume => AtomicOrdering::Acquire, // Compatibility: no Consume
        MemoryOrder::Acquire => AtomicOrdering::Acquire,
        MemoryOrder::Release => AtomicOrdering::Release,
        MemoryOrder::AcqRel => AtomicOrdering::AcquireRelease,
        MemoryOrder::SeqCst => AtomicOrdering::SequentiallyConsistent,
    }
}
