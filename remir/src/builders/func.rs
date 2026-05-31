use crate::{
    func::FunctionReference, insts::Instruction, module::Module, values::BaseSSAValue,
    writer::InstructionWriter,
};

pub fn build_call(
    module: &mut Module,
    label: FunctionReference,
    args: Vec<BaseSSAValue>,
    pure: bool,
    no_capture: bool,
    no_return: bool,
    fast_calling_conv: bool,
) -> Result<Option<BaseSSAValue>, ()> {
    let arguments = module.functions[label.id].arguments.clone();
    let return_type = module.functions[label.id].return_type.clone();

    let mut ind = 0;
    for arg in &args {
        if arg.value_type != arguments[ind].1 {
            return Err(());
        }

        ind += 1;
    }

    let inst = Instruction::Call {
        func_label: label,
        args,
        pure,
        no_capture,
        no_return,
        fast_calling_conv,
    };

    let val = module.write(inst);
    let has_return = !no_return && return_type.is_some();

    if has_return {
        Ok(Some(val.get()?))
    } else {
        Ok(None)
    }
}

pub fn build_ret(module: &mut Module, val: Option<BaseSSAValue>) -> Result<(), ()> {
    if val.is_none() {
        let inst = Instruction::RetNull;

        module.write(inst);
    } else {
        let inst = unsafe {
            Instruction::Ret {
                val: val.unwrap_unchecked(),
            }
        };

        module.write(inst);
    }

    Ok(())
}
