use std::hint::unreachable_unchecked;

use inkwell::{
    module::Linkage,
    types::StringRadix,
    values::{BasicValueEnum, IntValue},
};
use remir::{block::BlockInstruction, insts::Instruction, module::Module, values::ValueType};

use crate::{LLVMBridge, llvm_to_base, utils::LLVMBasicValue};

pub fn bridge_llvm_const_instruction(
    instruction: BlockInstruction,
    bridge: &mut LLVMBridge,
    module: &mut Module,
) -> Result<Option<LLVMBasicValue>, ()> {
    let res: Option<BasicValueEnum<'static>> = match &instruction.instruction {
        Instruction::ConstInt { val, size, signed } => {
            let ty = bridge
                .type_storage
                .convert(ValueType::Int(*signed, *size))
                .into_int_type();

            let res = ty
                .const_int_from_string(&val.to_string(), StringRadix::Decimal)
                .unwrap();

            Some(res.into())
        }

        Instruction::ConstFloat { val, size } => {
            let ty = bridge
                .type_storage
                .convert(ValueType::Float(*size))
                .into_float_type();

            let res = unsafe { ty.const_float_from_string(&val.to_string()) };

            Some(res.into())
        }

        Instruction::ConstPointer { addr } => {
            let int_ty = bridge
                .type_storage
                .convert(ValueType::Int(false, 64))
                .into_int_type();

            let ty = bridge
                .type_storage
                .convert(ValueType::Pointer(Box::new(ValueType::Unknown)))
                .into_pointer_type();

            let addr = int_ty.const_int(*addr as u64, false);

            let ptr = llvm_to_base!(bridge.builder.build_int_to_ptr(addr, ty, ""));

            Some(ptr.into())
        }

        Instruction::ConstString { str } => {
            let bytes = str.as_bytes();
            let byte_type = bridge
                .type_storage
                .convert(ValueType::Int(false, 8))
                .into_int_type();

            let arr_type = byte_type.array_type((bytes.len() + 1) as u32);

            let global = bridge.modules[&module.name].add_global(arr_type, None, "");

            global.set_linkage(Linkage::Private);
            global.set_constant(true);
            global.set_unnamed_addr(true);

            let mut vals: Vec<IntValue> = bytes
                .iter()
                .map(|b| byte_type.const_int(*b as u64, false))
                .collect();

            vals.push(byte_type.const_zero());

            global.set_initializer(&byte_type.const_array(&vals));

            Some(global.as_pointer_value().into())
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}
