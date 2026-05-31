use std::{collections::HashMap, mem::transmute, rc::Rc};

use inkwell::{builder::Builder, context::Context, llvm_sys::LLVMBasicBlock, types::VoidType};
use remir::{block::BlockReference, func::FunctionReference};

use crate::utils::{LLVMBasicValue, LLVMFunction, LLVMModule, LLVMSiblingObject, LLVMVoidType};

pub mod utils;

pub struct LLVMBridge {
    pub blocks: HashMap<BlockReference, LLVMBasicBlock>,
    pub values: HashMap<usize, LLVMBasicValue>,

    pub functions: HashMap<FunctionReference, LLVMFunction>,

    pub modules: HashMap<String, LLVMModule>,
    pub ctx: Rc<Context>,

    pub builder: Builder<'static>,
    pub void_type: LLVMVoidType,
}

impl LLVMBridge {
    pub fn new(ctx: Rc<Context>) -> Self {
        LLVMBridge {
            blocks: HashMap::new(),
            values: HashMap::new(),
            functions: HashMap::new(),
            modules: HashMap::new(),
            ctx: ctx.clone(),
            builder: unsafe { transmute::<Builder, Builder<'static>>(ctx.create_builder()) },
            void_type: LLVMSiblingObject::new(
                unsafe { transmute::<VoidType, VoidType<'static>>(ctx.void_type()) },
                &ctx,
            ),
        }
    }
}
