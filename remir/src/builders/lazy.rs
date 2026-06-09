use crate::{
    block::BlockReference,
    errs::RemirResult,
    insts::Instruction,
    module::Module,
    return_err,
    values::{BaseSSAValue, ValueType},
    writer::InstructionWriter,
};

pub fn build_lazy_load(
    module: &mut Module,
    block: BlockReference,
    variable_name: String,
    variable_type: ValueType,
) -> RemirResult<BaseSSAValue> {
    if !module.blocks[block.id]
        .variables
        .contains_key(&variable_name)
    {
        return_err!("variable not found")
    }

    let inst = Instruction::LazyLoad {
        block,
        variable_name,
        ty: variable_type,
    };

    module.write(inst).get()
}
