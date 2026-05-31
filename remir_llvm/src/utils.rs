use std::ops::Deref;
#[cfg(not(feature = "no_sibling_safety"))]
use std::rc::Rc;

#[cfg(not(feature = "no_sibling_safety"))]
use inkwell::context::Context;
use inkwell::{
    basic_block::BasicBlock,
    types::{BasicMetadataTypeEnum, BasicTypeEnum, IntType, PointerType},
    values::{BasicValueEnum, FunctionValue},
};

pub type LLVMBlock = LLVMSiblingObject<BasicBlock<'static>>;
pub type LLVMBasicValue = LLVMSiblingObject<BasicValueEnum<'static>>;
pub type LLVMFunction = LLVMSiblingObject<FunctionValue<'static>>;

pub type LLVMType = LLVMSiblingObject<IntType<'static>>;
pub type LLVMPointerType = LLVMSiblingObject<PointerType<'static>>;
pub type LLVMTypeEnum = LLVMSiblingObject<BasicTypeEnum<'static>>;
pub type LLVMMetadataEnum = LLVMSiblingObject<BasicMetadataTypeEnum<'static>>;

pub struct LLVMSiblingObject<T: Clone> {
    pub innner: T,

    /// Allows for the reference to make sure it lives
    #[cfg(not(feature = "no_sibling_safety"))]
    pub safety_hold: Rc<Context>,
}

impl<T: Clone> LLVMSiblingObject<T> {
    #[cfg(not(feature = "no_sibling_safety"))]
    pub fn new(val: T, held: &Rc<Context>) -> Self {
        LLVMSiblingObject {
            innner: unsafe { std::mem::transmute(val) },
            safety_hold: held.clone(),
        }
    }

    #[cfg(feature = "no_sibling_safety")]
    pub fn new(val: T) -> Self {
        LLVMSiblingObject {
            inner: unsafe { std::mem::transmute(val) },
        }
    }
}

impl<T: Clone> Deref for LLVMSiblingObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.innner
    }
}
