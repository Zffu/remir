use std::{collections::HashMap, mem::transmute, rc::Rc};

use inkwell::{builder::Builder, context::Context, types::VoidType};
use remir::{
    block::{Block, BlockReference},
    func::FunctionReference,
    module::Module,
};

use crate::{
    inst::bridge_llvm_instruction,
    types::LLVMTypeStorage,
    utils::{LLVMBasicValue, LLVMBlock, LLVMFunction, LLVMModule, LLVMSiblingObject, LLVMVoidType},
};

pub mod inst;
pub mod types;
pub mod utils;

#[macro_export]
macro_rules! llvm_to_base {
    ($expr: expr) => {
        match $expr {
            Ok(v) => v,
            Err(_) => return Err(()),
        }
    };
}

#[macro_export]
macro_rules! llvm_to_base_returnless {
    ($expr: expr) => {
        match $expr {
            Ok(_) => {}
            Err(_) => return Err(()),
        }
    };
}

pub struct LLVMBridge {
    pub blocks: HashMap<BlockReference, LLVMBlock>,
    pub values: HashMap<usize, LLVMBasicValue>,

    pub functions: HashMap<FunctionReference, LLVMFunction>,

    pub type_storage: LLVMTypeStorage,

    pub modules: HashMap<String, LLVMModule>,
    pub ctx: Rc<Context>,

    pub builder: Builder<'static>,
    pub void_type: LLVMVoidType,
}

pub fn build_llvm_block(
    bridge: &mut LLVMBridge,
    block: &Block,
    module: &mut Module,
) -> Result<(), ()> {
    bridge
        .builder
        .position_at_end(bridge.blocks[&block.reference].inner.clone());

    let func_ref = module.block_to_function[&block.reference].clone();

    for inst in &block.instructions {
        let res = bridge_llvm_instruction(inst.clone(), bridge, func_ref.clone(), module).unwrap();

        if res.is_some() {
            unsafe {
                bridge
                    .values
                    .insert(inst.get()?.inst_ind, res.unwrap_unchecked())
            };
        }
    }

    Ok(())
}

impl LLVMBridge {
    pub fn new(ctx: Rc<Context>) -> Self {
        LLVMBridge {
            blocks: HashMap::new(),
            values: HashMap::new(),
            functions: HashMap::new(),
            type_storage: LLVMTypeStorage::new(&ctx),
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
