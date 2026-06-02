use crate::{
    block::BlockReference,
    insts::Instruction,
    module::Module,
    values::{
        BaseSSAValue, ValueType, float::SSAFloatValue, int::SSAIntValue, structs::SSAStructValue,
    },
    writer::InstructionWriter,
};

pub fn build_bit_cast(
    module: &mut Module,
    src: BaseSSAValue,
    into: ValueType,
) -> Result<BaseSSAValue, ()> {
    if src.value_type == into {
        return Err(()); // Cannot use bit cast where source == into type
    }

    let inst = Instruction::BitCast { src, into };

    module.write(inst).get()
}

pub fn build_select(
    module: &mut Module,
    cond: SSAIntValue,
    true_val: BaseSSAValue,
    false_val: BaseSSAValue,
) -> Result<BaseSSAValue, ()> {
    cond.enforces_boolean()?;

    let inst = Instruction::Select {
        cond,
        true_val,
        false_val,
    };

    module.write(inst).get()
}

pub fn int_to_float(
    module: &mut Module,
    val: SSAIntValue,
    into: ValueType,
) -> Result<SSAFloatValue, ()> {
    if let ValueType::Float(_) = into {
        let inst = Instruction::IntToFloat { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        Err(()) // Target type is not a float
    }
}

pub fn float_to_int(
    module: &mut Module,
    val: SSAFloatValue,
    into: ValueType,
) -> Result<SSAIntValue, ()> {
    if let ValueType::Int(_, _) = into {
        let inst = Instruction::FloatToInt { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        Err(()) // Target type is not an int
    }
}

pub fn build_int_extend(
    module: &mut Module,
    val: SSAIntValue,
    into: ValueType,
) -> Result<SSAIntValue, ()> {
    if let ValueType::Int(_, size) = &into {
        if val.size >= *size {
            return Err(()); // Use int truncate instead
        }

        let inst = Instruction::IntExtend { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        Err(()) // Target type is not an int
    }
}

pub fn build_int_truncate(
    module: &mut Module,
    val: SSAIntValue,
    into: ValueType,
) -> Result<SSAIntValue, ()> {
    if let ValueType::Int(_, size) = &into {
        if val.size <= *size {
            return Err(()); // Use int extend instead
        }

        let inst = Instruction::IntTruncate { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        Err(()) // Target type is not an int
    }
}

pub fn build_float_extend(
    module: &mut Module,
    val: SSAFloatValue,
    into: ValueType,
) -> Result<SSAFloatValue, ()> {
    if let ValueType::Float(size) = &into {
        if val.size >= *size {
            return Err(()); // Use float truncate instead
        }

        let inst = Instruction::FloatExtend { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        Err(()) // Target type is not a float
    }
}

pub fn build_float_truncate(
    module: &mut Module,
    val: SSAFloatValue,
    into: ValueType,
) -> Result<SSAFloatValue, ()> {
    if let ValueType::Float(size) = &into {
        if val.size <= *size {
            return Err(()); // Use float extend instead
        }

        let inst = Instruction::FloatTruncate { val, into };

        let val = module.write(inst).get()?;

        val.try_into()
    } else {
        Err(()) // Target type is not a float
    }
}

pub fn build_int_change_size(
    module: &mut Module,
    val: SSAIntValue,
    into: ValueType,
) -> Result<SSAIntValue, ()> {
    if let ValueType::Int(_, size) = &into {
        if val.size > *size {
            build_int_truncate(module, val, into)
        } else {
            build_int_extend(module, val, into)
        }
    } else {
        Err(()) // Target type is not an int
    }
}

pub fn build_float_change_size(
    module: &mut Module,
    val: SSAFloatValue,
    into: ValueType,
) -> Result<SSAFloatValue, ()> {
    if let ValueType::Float(size) = &into {
        if val.size > *size {
            build_float_truncate(module, val, into)
        } else {
            build_float_extend(module, val, into)
        }
    } else {
        Err(()) // Target type is not an float
    }
}

pub fn build_extract_value(
    module: &mut Module,
    struct_val: SSAStructValue,
    index: usize,
) -> Result<BaseSSAValue, ()> {
    let inst = Instruction::ExtractValue { struct_val, index };

    module.write(inst).get()
}

pub fn build_insert_value(
    module: &mut Module,
    struct_val: SSAStructValue,
    index: usize,
    val: BaseSSAValue,
) -> Result<(), ()> {
    if struct_val.fields[index] != val.value_type {
        return Err(()); // Cannot put into diff field type
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
    cond: SSAIntValue,
    default: BaseSSAValue,
    else_block: BlockReference,
    cases: Vec<(SSAIntValue, BaseSSAValue)>,
) -> Result<BaseSSAValue, ()> {
    cond.enforces_boolean()?;

    for case in &cases {
        if case.1.value_type != default.value_type {
            return Err(()); // Every choice must be of the same type
        }
    }

    let inst = Instruction::Switch {
        cond,
        default,
        else_block,
        cases,
    };

    module.write(inst).get()
}
