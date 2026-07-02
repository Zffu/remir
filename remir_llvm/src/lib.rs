use std::{
    collections::{HashMap, HashSet},
    mem::transmute,
    path::PathBuf,
    rc::Rc,
};

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine},
    types::{BasicMetadataTypeEnum, BasicType, VoidType},
};
use remir::{
    OptimizationLevel,
    block::{Block, BlockReference},
    errs::{RemirError, RemirResult},
    func::{Function, FunctionReference},
    module::Module,
    values::ValueType,
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
            Err(e) => {
                panic!("Caught {}", e);
            }
        }
    };
}

#[macro_export]
macro_rules! llvm_to_base_returnless {
    ($expr: expr) => {
        match $expr {
            Ok(_) => {}
            Err(e) => {
                panic!("Caught {}", e);
            }
        }
    };
}

pub struct LLVMBridge {
    pub blocks: HashMap<BlockReference, LLVMBlock>,
    pub built_blocks: HashSet<BlockReference>,
    pub values: HashMap<usize, LLVMBasicValue>,

    pub functions: HashMap<FunctionReference, LLVMFunction>,

    pub type_storage: LLVMTypeStorage,

    pub modules: HashMap<String, LLVMModule>,
    pub ctx: Rc<Context>,

    pub builder: Builder<'static>,
    pub void_type: LLVMVoidType,
}

pub fn compile_llvm(
    bridge: &mut LLVMBridge,
    module: &mut Module,
    optimization_level: OptimizationLevel,
    path: PathBuf,
    pie: bool,
) -> RemirResult<()> {
    match build_llvm(bridge, module) {
        Ok(_) => {}
        Err(_) => return Err(RemirError::new("Build LLVM failed!")),
    }

    let check = bridge.modules[&module.name].verify();

    if check.is_err() {
        bridge.modules[&module.name].print_to_stderr();

        return Err(RemirError::new(&format!(
            "LLVM Check Error: {}",
            check.unwrap_err()
        )));
    }

    let module = bridge.modules[&module.name].clone();

    Target::initialize_native(&InitializationConfig::default()).unwrap();

    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).unwrap();

    let level = match optimization_level {
        OptimizationLevel::None => inkwell::OptimizationLevel::None,
        OptimizationLevel::Less => inkwell::OptimizationLevel::Less,
        OptimizationLevel::Default => inkwell::OptimizationLevel::Default,
        OptimizationLevel::Aggressive => inkwell::OptimizationLevel::Aggressive,
    };

    let reloc_mode;

    if pie {
        reloc_mode = RelocMode::PIC
    } else {
        reloc_mode = RelocMode::Default;
    }

    let machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            level,
            reloc_mode,
            CodeModel::Default,
        )
        .unwrap();

    machine
        .write_to_file(&module, inkwell::targets::FileType::Object, &path)
        .unwrap();

    Ok(())
}

pub fn build_llvm(bridge: &mut LLVMBridge, module: &mut Module) -> Result<(), ()> {
    let m = unsafe {
        transmute::<inkwell::module::Module, inkwell::module::Module<'static>>(
            bridge.ctx.create_module(&module.name),
        )
    };

    bridge
        .modules
        .insert(module.name.clone(), LLVMModule::new(m, &bridge.ctx));

    for func in module.functions.clone() {
        bridge_llvm_function(bridge, &func, module);
    }

    for block in module.blocks.clone() {
        build_llvm_block(bridge, &block, module)?;
    }

    Ok(())
}

pub fn build_llvm_block(
    bridge: &mut LLVMBridge,
    block: &Block,
    module: &mut Module,
) -> Result<(), ()> {
    if bridge.built_blocks.contains(&block.reference) {
        return Ok(());
    }

    for dependency in &block.dependencies {
        build_llvm_block(bridge, &module.blocks[dependency.id].clone(), module)?;
    }

    bridge
        .builder
        .position_at_end(bridge.blocks[&block.reference].inner.clone());

    let func_ref = module.block_to_function[&block.reference].clone();

    for inst in &block.instructions {
        let res = bridge_llvm_instruction(inst.clone(), bridge, func_ref.clone(), module)?;

        if res.is_some() {
            unsafe {
                bridge
                    .values
                    .insert(inst.get().unwrap().inst_ind, res.unwrap_unchecked())
            };
        }
    }

    bridge.built_blocks.insert(block.reference.clone());

    Ok(())
}

pub fn bridge_llvm_function(bridge: &mut LLVMBridge, func: &Function, module: &mut Module) {
    let mut arguments: Vec<BasicMetadataTypeEnum> = vec![];

    for arg in &func.arguments {
        arguments.push(bridge.type_storage.convert(arg.clone()).inner.into());
    }

    let llvm_func;

    if func.return_type != ValueType::Void {
        let ret_type = bridge.type_storage.convert(func.return_type.clone()).inner;

        llvm_func = ret_type.fn_type(&arguments, func.triple_dot_position.is_some());
    } else {
        llvm_func = bridge
            .void_type
            .fn_type(&arguments, func.triple_dot_position.is_some());
    }

    let llvm_f = bridge.modules[&module.name].add_function(&func.reference.name, llvm_func, None);

    for block in &func.blocks {
        let bb = bridge.ctx.append_basic_block(llvm_f.clone(), &block.name);
        let ctx = bridge.ctx.clone();

        let b = LLVMBlock::new_owned(
            unsafe { transmute::<BasicBlock, BasicBlock<'static>>(bb) },
            ctx,
        );

        bridge.blocks.insert(block.clone(), b);
    }

    bridge.functions.insert(
        func.reference.clone(),
        LLVMFunction::new(llvm_f, &bridge.ctx),
    );
}

impl LLVMBridge {
    pub fn new() -> Self {
        let ctx = Context::create();
        let ctx = Rc::new(ctx);

        LLVMBridge {
            blocks: HashMap::new(),
            built_blocks: HashSet::new(),
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
