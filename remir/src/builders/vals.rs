use crate::{
    block::BlockReference,
    errs::RemirResult,
    insts::Instruction,
    module::Module,
    return_err,
    values::{
        BaseSSAValue, ValueType, float::SSAFloatValue, int::SSAIntValue, ptr::SSAPointerValue,
        structs::SSAStructValue,
    },
    writer::InstructionWriter,
};

pub fn build_bit_cast(
    module: &mut Module,
    src: BaseSSAValue,
    into: ValueType,
) -> RemirResult<BaseSSAValue> {
    if src.value_type == into {
        return_err!("Cannot use bitcast with a same-type value and into"); // Cannot use bit cast where source == into type
    }

    let inst = Instruction::BitCast { src, into };

    module.write(inst).get()
}

pub fn build_select(
    module: &mut Module,
    cond: SSAIntValue,
    true_val: BaseSSAValue,
    false_val: BaseSSAValue,
) -> RemirResult<BaseSSAValue> {
    cond.enforces_boolean()?;

    let inst = Instruction::Select {
        cond,
        true_val,
        false_val,
    };

    module.write(inst).get()
}

pub fn build_int_to_float(
    module: &mut Module,
    val: SSAIntValue,
    into: ValueType,
) -> RemirResult<SSAFloatValue> {
    if let ValueType::Float(_) = into {
        let inst = Instruction::IntToFloat { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        return_err!("target type is not a float") // Target type is not a float
    }
}

pub fn build_float_to_int(
    module: &mut Module,
    val: SSAFloatValue,
    into: ValueType,
) -> RemirResult<SSAIntValue> {
    if let ValueType::Int(_, _) = into {
        let inst = Instruction::FloatToInt { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        return_err!("target type is not an int") // Target type is not an int
    }
}

pub fn build_int_extend(
    module: &mut Module,
    val: SSAIntValue,
    into: ValueType,
) -> RemirResult<SSAIntValue> {
    if let ValueType::Int(_, size) = &into {
        if val.size >= *size {
            return_err!("val.size >= size for int extend! use int_truncate instead"); // Use int truncate instead
        }

        let inst = Instruction::IntExtend { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        return_err!("target type is not an int") // Target type is not an int
    }
}

pub fn build_int_truncate(
    module: &mut Module,
    val: SSAIntValue,
    into: ValueType,
) -> RemirResult<SSAIntValue> {
    if let ValueType::Int(_, size) = &into {
        if val.size <= *size {
            return_err!("val.size <= size for int truncate! use int_extend instead"); // Use int extend instead
        }

        let inst = Instruction::IntTruncate { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        return_err!("target type is not an int") // Target type is not an int
    }
}

pub fn build_float_extend(
    module: &mut Module,
    val: SSAFloatValue,
    into: ValueType,
) -> RemirResult<SSAFloatValue> {
    if let ValueType::Float(size) = &into {
        if val.size >= *size {
            return_err!("val.size >= size for float extend! use float_truncate instead"); // Use float truncate instead
        }

        let inst = Instruction::FloatExtend { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        return_err!("target type is not a float") // Target type is not a float
    }
}

pub fn build_float_truncate(
    module: &mut Module,
    val: SSAFloatValue,
    into: ValueType,
) -> RemirResult<SSAFloatValue> {
    if let ValueType::Float(size) = &into {
        if val.size <= *size {
            return_err!("val.size <= size for float truncate! use float_extend instead"); // Use float extend instead
        }

        let inst = Instruction::FloatTruncate { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        return_err!("target type is not a float") // Target type is not a float
    }
}

pub fn build_int_change_size(
    module: &mut Module,
    val: SSAIntValue,
    into: ValueType,
) -> RemirResult<SSAIntValue> {
    if let ValueType::Int(_, size) = &into {
        if val.size > *size {
            build_int_truncate(module, val, into)
        } else {
            build_int_extend(module, val, into)
        }
    } else {
        return_err!("target type is not an int") // Target type is not an int
    }
}

pub fn build_float_change_size(
    module: &mut Module,
    val: SSAFloatValue,
    into: ValueType,
) -> RemirResult<SSAFloatValue> {
    if let ValueType::Float(size) = &into {
        if val.size > *size {
            build_float_truncate(module, val, into)
        } else {
            build_float_extend(module, val, into)
        }
    } else {
        return_err!("target type is not a float") // Target type is not an float
    }
}

pub fn build_extract_value(
    module: &mut Module,
    struct_val: SSAStructValue,
    index: usize,
) -> RemirResult<BaseSSAValue> {
    let inst = Instruction::ExtractValue { struct_val, index };

    module.write(inst).get()
}

pub fn build_insert_value(
    module: &mut Module,
    struct_val: SSAStructValue,
    index: usize,
    val: BaseSSAValue,
) -> RemirResult<()> {
    if struct_val.fields[index] != val.value_type {
        return_err!("Cannot put val into a different typed field"); // Cannot put into diff field type
    }

    let inst = Instruction::InsertValue {
        struct_val,
        index,
        val,
    };

    module.write(inst);
    Ok(())
}

pub fn build_switch(
    module: &mut Module,
    value: SSAIntValue,
    else_block: BlockReference,
    cases: Vec<(SSAIntValue, BlockReference)>,
) -> RemirResult<()> {
    for case in &cases {
        if case.0.base.value_type != value.base.value_type {
            return_err!("every choice must be of the same type in a switch!"); // Every choice must be of the same type
        }
    }

    let inst = Instruction::Switch {
        value,
        else_block,
        cases,
    };

    module.write(inst);
    Ok(())
}

pub fn build_int_to_ptr(
    module: &mut Module,
    val: SSAIntValue,
    into: ValueType,
) -> RemirResult<SSAPointerValue> {
    if let ValueType::Pointer(_) = &into {
        let inst = Instruction::IntToPtr { val, into };

        let val = module.write(inst).get()?;

        SSAPointerValue::try_from(val)
    } else {
        return_err!("expected pointer type to do int -> ptr")
    }
}
