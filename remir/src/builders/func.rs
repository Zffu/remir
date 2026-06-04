use crate::{
    errs::RemirResult, func::FunctionReference, insts::Instruction, module::Module, return_err,
    values::BaseSSAValue, writer::InstructionWriter,
};

pub fn build_call(
    module: &mut Module,
    label: FunctionReference,
    args: Vec<BaseSSAValue>,
    pure: bool,
    no_return: bool,
    fast_calling_conv: bool,
) -> RemirResult<Option<BaseSSAValue>> {
    let arguments = module.functions[label.id].arguments.clone();
    let return_type = module.functions[label.id].return_type.clone();

    let mut ind = 0;
    for arg in &args {
        if arg.value_type != arguments[ind] {
            return_err!("Argument types do not match the declaration")
        }

        ind += 1;
    }

    let inst = Instruction::Call {
        func_label: label,
        args,
        pure,
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

pub fn build_ret(module: &mut Module, val: Option<BaseSSAValue>) {
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
}
pub fn build_argument_grab(module: &mut Module, index: usize) -> RemirResult<BaseSSAValue> {
    let inst = Instruction::GrabArgument { index };

    module.write(inst).get()
}
