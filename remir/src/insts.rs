//! Definitions of the instruction set of Remir

use crate::{
    utils::operators::{CompareOperator, MathOperator},
    values::{
        BaseSSAValue, ValueType, float::SSAFloatValue, int::SSAIntValue, ptr::SSAPointerValue,
    },
};

/// Represents an instruction in the MIR.
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
        branch: usize,
    },

    Condbr {
        cond: SSAIntValue,
        true_label: usize,
        false_label: usize,
    },

    IndirectBranch {
        target: SSAPointerValue,
    },

    Phi {
        label_set: Vec<(usize, BaseSSAValue)>,
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
        struct_val: BaseSSAValue,
        index: usize,
    },

    InsertValue {
        struct_val: BaseSSAValue,
        index: usize,
        val: BaseSSAValue,
    },
}
