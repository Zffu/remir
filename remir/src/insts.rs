//! Definitions of the instruction set of Remir

use crate::{
    block::BlockReference,
    utils::{
        atomic::MemoryOrder,
        operators::{CompareOperator, MathOperator},
    },
    values::{
        BaseSSAValue, ValueType, consts::ConstantData, float::SSAFloatValue, int::SSAIntValue,
        ptr::SSAPointerValue, structs::SSAStructValue,
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
    #[deprecated = "const instructions might get removed soon"]
    AllocConst {
        size: usize,
        val_type: ValueType,
    },

    Alloc {
        size: SSAIntValue,
        val_type: ValueType,
    },

    AllocUntyped {
        size: SSAIntValue,
    },

    #[deprecated = "const instructions might get removed soon"]
    AllocaConst {
        size: usize,
        val_type: ValueType,
    },

    Alloca {
        size: SSAIntValue,
        val_type: ValueType,
    },

    AllocaUntyped {
        size: SSAIntValue,
    },

    Free {
        ptr: SSAPointerValue,
    },

    #[deprecated = "const instructions might get removed soon"]
    GepConst {
        base: SSAPointerValue,
        offset: usize,
    },

    Gep {
        base: SSAPointerValue,
        offset: SSAIntValue,
    },

    #[deprecated = "const instructions might get removed soon"]
    LoadIndexedConst {
        base: SSAPointerValue,
        index: usize,
    },

    LoadIndexed {
        base: SSAPointerValue,
        index: SSAIntValue,
    },

    #[deprecated = "const instructions might get removed soon"]
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
            Self::Alloc { size: _, val_type } => {
                Some(ValueType::Pointer(Box::new(val_type.clone())))
            }
            Self::AllocConst { size: _, val_type } => {
                Some(ValueType::Pointer(Box::new(val_type.clone())))
            }
            Self::AllocUntyped { .. } => Some(ValueType::Pointer(Box::new(ValueType::Unknown))),

            Self::Alloca { size: _, val_type } => {
                Some(ValueType::Pointer(Box::new(val_type.clone())))
            }
            Self::AllocaConst { size: _, val_type } => {
                Some(ValueType::Pointer(Box::new(val_type.clone())))
            }
            Self::AllocaUntyped { .. } => Some(ValueType::Pointer(Box::new(ValueType::Unknown))),

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
            Self::ConstPointer { .. } => Some(ValueType::Pointer(Box::new(ValueType::Unknown))),
            Self::Copy { val } => Some(val.value_type.clone()),
            Self::ExtractValue { struct_val, index } => Some(struct_val.fields[*index].clone()),
            Self::FloatExtend { val: _, into } => Some(into.clone()),
            Self::FloatToInt { val: _, into } => Some(into.clone()),
            Self::FloatTruncate { val: _, into } => Some(into.clone()),
            Self::Gep { base, offset: _ } => Some(base.base.value_type.clone()),
            Self::GepConst { base, offset: _ } => Some(base.base.value_type.clone()),
            Self::IntExtend { val: _, into } => Some(into.clone()),
            Self::IntToFloat { val: _, into } => Some(into.clone()),
            Self::IntTruncate { val: _, into } => Some(into.clone()),
            Self::Load { source } => Some(source.inner_type.clone()),
            Self::LoadAtomic {
                source,
                ordering: _,
            } => Some(source.inner_type.clone()),
            Self::LoadIndexed { base, index: _ } => Some(base.inner_type.clone()),
            Self::LoadIndexedConst { base, index: _ } => Some(base.inner_type.clone()),
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

    pub fn get_output_constant(&self) -> ConstantData {
        match self {
            Self::ConstInt {
                val,
                size: _,
                signed: _,
            } => ConstantData::Int(*val),

            Self::ConstFloat {
                val,
                size: _,
                signed: _,
            } => ConstantData::Float(*val),

            Self::ConstPointer { addr } => ConstantData::Pointer(*addr),

            _ => ConstantData::None,
        }
    }
}
