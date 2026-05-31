use crate::{
    insts::Instruction,
    module::Module,
    values::{BaseSSAValue, ValueType, int::SSAIntValue},
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
