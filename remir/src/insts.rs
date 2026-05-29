//! Definitions of the instruction set of Remir

use crate::{
    block::BlockReference,
    utils::{
        atomic::MemoryOrder,
        operators::{CompareOperator, MathOperator},
    },
    values::{
        BaseSSAValue, ValueType, float::SSAFloatValue, int::SSAIntValue, ptr::SSAPointerValue,
        structs::SSAStructValue,
    },
};

/// Represents an instruction in the MIR.
#[derive(Clone)]
pub enum Instruction {
    // Constant instructions
    ConstInt {
        val: i128,
        size: usize,
        signed: bool,
    },

    ConstFloat {
        val: f64,
        size: usize,
        signed: bool,
    },

    ConstPointer {
        addr: usize,
    },

    // Register instructions
    Copy {
        val: BaseSSAValue,
    },

    Load {
        source: SSAPointerValue,
    },

    Store {
        destination: SSAPointerValue,
        source: BaseSSAValue,
    },

    // Math instructions
    MathOperationInt {
        a: SSAIntValue,
        b: SSAIntValue,
        op: MathOperator,

        signed: bool,
        signed_wrap: bool,
        unsigned_wrap: bool,
        fast: bool,
    },

    MathOperationFloat {
        a: SSAFloatValue,
        b: SSAFloatValue,
        op: MathOperator,

        signed: bool,
        signed_wrap: bool,
        unsigned_wrap: bool,
        fast: bool,
    },

    // Cmp instructions
    CompareOperationInt {
        a: SSAIntValue,
        b: SSAIntValue,

        op: CompareOperator,

        signed: bool,
    },

    CompareOperationFloat {
        a: SSAFloatValue,
        b: SSAFloatValue,

        op: CompareOperator,

        signed: bool,
    },

    // Branch instructions
    UncondBr {
        branch: BlockReference,
    },

    Condbr {
        cond: SSAIntValue,
        true_label: BlockReference,
        false_label: BlockReference,
    },

    IndirectBranch {
        target: SSAPointerValue,
    },

    Phi {
        label_set: Vec<(BlockReference, BaseSSAValue)>,
    },

    // Function instructions
    Call {
        func_label: usize,
        args: Vec<BaseSSAValue>,

        pure: bool,
        no_capture: bool,
        no_return: bool,

        fast_calling_conv: bool,
    },

    RetNull,
    Ret {
        val: BaseSSAValue,
    },

    // Memory instructions
    AllocConst {
        size: usize,
    },

    Alloc {
        size: SSAIntValue,
    },

    AllocaConst {
        size: usize,
    },

    Alloca {
        size: SSAIntValue,
    },

    Free {
        ptr: SSAPointerValue,
    },

    GepConst {
        base: SSAPointerValue,
        offset: usize,
    },

    Gep {
        base: SSAPointerValue,
        offset: SSAIntValue,
    },

    LoadIndexedConst {
        base: SSAPointerValue,
        index: usize,
    },

    LoadIndexed {
        base: SSAPointerValue,
        index: SSAIntValue,
    },

    StoreIndexedConst {
        base: SSAPointerValue,
        index: usize,
        val: BaseSSAValue,
    },

    StoreIndexed {
        base: SSAPointerValue,
        index: usize,
        val: BaseSSAValue,
    },

    // Value manipulation instructions
    BitCast {
        src: BaseSSAValue,
        into: ValueType,
    },

    Select {
        cond: SSAIntValue,
        true_val: BaseSSAValue,
        false_val: BaseSSAValue,
    },

    // Number instructions
    IntToFloat {
        val: SSAIntValue,
        into: ValueType,
    },

    FloatToInt {
        val: SSAFloatValue,
        into: ValueType,
    },

    IntExtend {
        val: SSAIntValue,
        into: ValueType,
    },

    IntTruncate {
        val: SSAIntValue,
        into: ValueType,
    },

    FloatExtend {
        val: SSAFloatValue,
        into: ValueType,
    },

    FloatTruncate {
        val: SSAFloatValue,
        into: ValueType,
    },

    // Struct instructions
    ExtractValue {
        struct_val: SSAStructValue,
        index: usize,
    },

    InsertValue {
        struct_val: SSAStructValue,
        index: usize,
        val: BaseSSAValue,
    },

    Switch {
        cond: SSAIntValue,
        default: BaseSSAValue,
        cases: Vec<(i128, BaseSSAValue)>,

        min_neg: i128,
        max: i128,
    },

    // Atomic instructions
    LoadAtomic {
        source: SSAPointerValue,
        ordering: MemoryOrder,
    },

    StoreAtomic {
        dest: SSAPointerValue,
        val: BaseSSAValue,
        ordering: MemoryOrder,
    },

    Fence {
        ordering: MemoryOrder,
    },

    /// SSA Hints
    Unreachable,

    Crash {
        message: Option<String>,
    },

    Assume {
        val: SSAIntValue,
    },
}

impl Instruction {
    /// Checks if the given instruction is supposed to return a value
    pub fn outputs_value(&self) -> bool {
        match self {
            Self::Assume { .. } => false,
            Self::Condbr { .. } => false,
            Self::Crash { .. } => false,
            Self::Fence { .. } => false,
            Self::Free { .. } => false,
            Self::IndirectBranch { .. } => false,
            Self::InsertValue { .. } => false,
            Self::Ret { .. } => false,
            Self::RetNull => false,
            Self::Switch { .. } => false,
            Self::Store { .. } => false,
            Self::StoreAtomic { .. } => false,
            Self::StoreIndexed { .. } => false,
            Self::StoreIndexedConst { .. } => false,
            Self::UncondBr { .. } => false,
            Self::Unreachable => false,

            _ => true,
        }
    }

    pub fn get_output_type(&self) -> Option<ValueType> {
        match self {
            Self::Alloc { .. } => Some(ValueType::Pointer),
            Self::AllocConst { .. } => Some(ValueType::Pointer),
            Self::Alloca { .. } => Some(ValueType::Pointer),
            Self::AllocaConst { .. } => Some(ValueType::Pointer),
            Self::BitCast { src: _, into } => Some(into.clone()),
            Self::Call { .. } => todo!(),
            Self::CompareOperationFloat { .. } => Some(ValueType::Int(false, 1)),
            Self::CompareOperationInt { .. } => Some(ValueType::Int(false, 1)),
            Self::ConstFloat {
                val: _,
                size,
                signed,
            } => Some(ValueType::Float(*signed, *size)),
            Self::ConstInt {
                val: _,
                size,
                signed,
            } => Some(ValueType::Int(*signed, *size)),
            Self::ConstPointer { .. } => Some(ValueType::Pointer),
            Self::Copy { val } => Some(val.value_type.clone()),
            Self::ExtractValue { struct_val, index } => Some(struct_val.fields[*index].clone()),
            Self::FloatExtend { val: _, into } => Some(into.clone()),
            Self::FloatToInt { val: _, into } => Some(into.clone()),
            Self::FloatTruncate { val: _, into } => Some(into.clone()),
            Self::Gep { .. } => todo!(),
            Self::GepConst { .. } => todo!(),
            Self::IntExtend { val: _, into } => Some(into.clone()),
            Self::IntToFloat { val: _, into } => Some(into.clone()),
            Self::IntTruncate { val: _, into } => Some(into.clone()),
            Self::Load { .. } => todo!(),
            Self::LoadAtomic { .. } => todo!(),
            Self::LoadIndexed { .. } => todo!(),
            Self::LoadIndexedConst { .. } => todo!(),
            Self::MathOperationFloat {
                a,
                b: _,
                op: _,
                signed: _,
                signed_wrap: _,
                unsigned_wrap: _,
                fast: _,
            } => Some(a.base.value_type.clone()),

            Self::MathOperationInt {
                a,
                b: _,
                op: _,
                signed: _,
                signed_wrap: _,
                unsigned_wrap: _,
                fast: _,
            } => Some(a.base.value_type.clone()),

            Self::Phi { label_set } => Some(label_set[0].1.value_type.clone()),
            Self::Select {
                cond: _,
                true_val,
                false_val: _,
            } => Some(true_val.value_type.clone()),

            _ => None,
        }
    }
}
