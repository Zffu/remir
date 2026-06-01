use std::hint::unreachable_unchecked;

use inkwell::types::BasicType;
use remir::{block::BlockInstruction, insts::Instruction, module::Module, values::ValueType};

use crate::{LLVMBridge, llvm_to_base, llvm_to_base_returnless, utils::LLVMBasicValue};

pub fn bridge_llvm_mem_instruction(
    instruction: BlockInstruction,
    bridge: &mut LLVMBridge,
    module: &mut Module,
) -> Result<Option<LLVMBasicValue>, ()> {
    let res = match &instruction.instruction {
        Instruction::Alloc { size: _, val_type } => {
            let res = llvm_to_base!(
                bridge
                    .builder
                    .build_malloc(bridge.type_storage.convert(val_type.clone()).inner, "")
            );

            Some(res.into())
        }

        Instruction::Alloca { size: _, val_type } => {
            let res = llvm_to_base!(
                bridge
                    .builder
                    .build_alloca(bridge.type_storage.convert(val_type.clone()).inner, "")
            );

            Some(res.into())
        }

        Instruction::AllocUntyped { size } => {
            let size = bridge.values[&size.base.inst_ind].into_int_value();

            let malloc_ty = bridge
                .type_storage
                .convert(ValueType::Pointer(Box::new(ValueType::Unknown)))
                .fn_type(
                    &[bridge
                        .type_storage
                        .convert(ValueType::Int(false, 64))
                        .inner
                        .into()],
                    false,
                );

            let malloc = bridge.modules[&module.name]
                .get_function("malloc")
                .unwrap_or_else(|| {
                    bridge.modules[&module.name].add_function("malloc", malloc_ty, None)
                });

            let ptr = llvm_to_base!(bridge.builder.build_call(malloc, &[size.into()], ""));

            ptr.try_as_basic_value().basic()
        }

        Instruction::AllocaUntyped { size } => {
            let size = bridge.values[&size.base.inst_ind].into_int_value();

            let alloca_ty = bridge
                .type_storage
                .convert(ValueType::Pointer(Box::new(ValueType::Unknown)))
                .fn_type(
                    &[bridge
                        .type_storage
                        .convert(ValueType::Int(false, 64))
                        .inner
                        .into()],
                    false,
                );

            let alloca = bridge.modules[&module.name]
                .get_function("alloca")
                .unwrap_or_else(|| {
                    bridge.modules[&module.name].add_function("alloca", alloca_ty, None)
                });

            let ptr = llvm_to_base!(bridge.builder.build_call(alloca, &[size.into()], ""));

            ptr.try_as_basic_value().basic()
        }

        Instruction::Free { ptr } => {
            let ptr = bridge.values[&ptr.base.inst_ind].into_pointer_value();

            llvm_to_base_returnless!(bridge.builder.build_free(ptr));

            None
        }

        Instruction::Gep { base, offset } => {
            let ty = bridge.type_storage.convert(ValueType::Int(false, 8));

            let base = bridge.values[&base.base.inst_ind].into_pointer_value();
            let offset = bridge.values[&offset.base.inst_ind].into_int_value();

            let val =
                unsafe { llvm_to_base!(bridge.builder.build_gep(ty.inner, base, &[offset], "")) };

            Some(val.into())
        }

        Instruction::LoadIndexed { base, index } => {
            let ty = bridge.type_storage.convert(base.inner_type.clone());

            let base = bridge.values[&base.base.inst_ind].into_pointer_value();
            let offset = bridge.values[&index.base.inst_ind].into_int_value();

            let ptr = unsafe {
                llvm_to_base!(
                    bridge
                        .builder
                        .build_gep(ty.inner.clone(), base, &[offset], "")
                )
            };

            let val = llvm_to_base!(bridge.builder.build_load(ty.inner.clone(), ptr, ""));

            Some(val)
        }

        Instruction::StoreIndexed { base, index, val } => {
            let ty = bridge.type_storage.convert(base.inner_type.clone());
            let val = bridge.values[&val.inst_ind].inner.clone();

            let base = bridge.values[&base.base.inst_ind].into_pointer_value();
            let offset = bridge.values[&index.base.inst_ind].into_int_value();

            let ptr = unsafe {
                llvm_to_base!(
                    bridge
                        .builder
                        .build_gep(ty.inner.clone(), base, &[offset], "")
                )
            };

            llvm_to_base_returnless!(bridge.builder.build_store(ptr, val));

            None
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}
