use std::hint::unreachable_unchecked;

use inkwell::{
    module::Linkage,
    types::{BasicType, BasicTypeEnum, StringRadix},
    values::{
        ArrayValue, BasicValue, BasicValueEnum, FloatValue, IntValue, PointerValue, StructValue,
    },
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

        Instruction::ConstStruct { ty, values } => {
            let ty = bridge.type_storage.convert(ty.clone()).into_struct_type();
            let mut vals = vec![];
            let mut is_const = true;

            for value in values {
                let val = bridge.values[&value.inst_ind].clone().inner;

                if !val.is_const() {
                    is_const = false;
                }

                vals.push(val);
            }

            let val;

            if is_const {
                val = ty.const_named_struct(&vals);
            } else {
                let mut struct_val = ty.get_undef();

                for (i, v) in vals.iter().enumerate() {
                    struct_val = bridge
                        .builder
                        .build_insert_value(struct_val, *v, i as u32, "")
                        .expect("valid insert value")
                        .into_struct_value();
                }

                val = struct_val;
            }

            Some(val.into())
        }

        Instruction::ConstArray { values } => {
            let vals: Vec<BasicValueEnum<'_>> = values
                .iter()
                .map(|v| bridge.values[&v.inst_ind].inner.clone())
                .collect();

            let elem_ty = bridge
                .type_storage
                .convert(values[0].value_type.clone())
                .inner;

            let array = bridge_llvm_array(elem_ty, vals);

            Some(array.into())
        }

        Instruction::ConstArraySame { value, count } => {
            let mut vals = vec![];
            let v = bridge.values[&value.inst_ind].inner.clone();

            for _ in 0..*count {
                vals.push(v.clone());
            }

            let elem_ty = bridge.type_storage.convert(value.value_type.clone()).inner;

            let array = bridge_llvm_array(elem_ty, vals);

            Some(array.into())
        }

        _ => unsafe { unreachable_unchecked() },
    };

    if res.is_some() {
        return Ok(Some(LLVMBasicValue::new(res.unwrap(), &bridge.ctx)));
    }

    return Ok(None);
}

pub fn bridge_llvm_array<'a>(
    elem_ty: BasicTypeEnum<'a>,
    vals: Vec<BasicValueEnum<'a>>,
) -> ArrayValue<'a> {
    let array = match elem_ty.as_basic_type_enum() {
        BasicTypeEnum::IntType(ty) => {
            let vals: Vec<IntValue<'_>> = vals.iter().map(|v| v.into_int_value()).collect();
            ty.const_array(&vals).into()
        }

        BasicTypeEnum::FloatType(ty) => {
            let vals: Vec<FloatValue<'_>> = vals.iter().map(|v| v.into_float_value()).collect();
            ty.const_array(&vals).into()
        }

        BasicTypeEnum::ArrayType(ty) => {
            let vals: Vec<ArrayValue<'_>> = vals.iter().map(|v| v.into_array_value()).collect();
            ty.const_array(&vals).into()
        }

        BasicTypeEnum::PointerType(ty) => {
            let vals: Vec<PointerValue<'_>> = vals.iter().map(|v| v.into_pointer_value()).collect();
            ty.const_array(&vals).into()
        }

        BasicTypeEnum::StructType(ty) => {
            let vals: Vec<StructValue<'_>> = vals.iter().map(|v| v.into_struct_value()).collect();
            ty.const_array(&vals).into()
        }

        _ => panic!(),
    };

    array
}
