//! Definitions of the instruction set of Remir

use crate::{
    block::BlockReference,
    func::FunctionReference,
    misc::{CompareOperator, MathOperator, MemoryOrder},
    module::Module,
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
    },

    ConstPointer {
        addr: usize,
    },

    ConstString {
        str: String,
    },

    ConstStruct {
        ty: ValueType,
        values: Vec<BaseSSAValue>,
    },

    ConstArray {
        values: Vec<BaseSSAValue>,
    },

    ConstArraySame {
        value: BaseSSAValue,
        count: usize,
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
    Not {
        val: SSAIntValue,
    },

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
        destinations: Vec<BlockReference>,
    },

    Phi {
        label_set: Vec<(BlockReference, BaseSSAValue)>,
    },

    // Function instructions
    Call {
        func_label: FunctionReference,
        args: Vec<BaseSSAValue>,

        pure: bool,
        no_return: bool,

        fast_calling_conv: bool,
    },

    RetNull,
    Ret {
        val: BaseSSAValue,
    },

    GrabArgument {
        index: usize,
    },

    // Memory instructions
    Alloc {
        size: SSAIntValue,
        val_type: ValueType,
    },

    AllocUntyped {
        size: SSAIntValue,
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

    Gep {
        base: SSAPointerValue,
        offset: SSAIntValue,
    },

    LoadIndexed {
        base: SSAPointerValue,
        index: SSAIntValue,
    },

    StoreIndexed {
        base: SSAPointerValue,
        index: SSAIntValue,
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
        value: SSAIntValue,
        else_block: BlockReference,
        cases: Vec<(SSAIntValue, BlockReference)>,
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
    pub fn outputs_value(&self, module: &mut Module) -> bool {
        match self {
            Self::Assume { .. } => false,
            Self::Condbr { .. } => false,
            Self::Crash { .. } => false,
            Self::Free { .. } => false,
            Self::IndirectBranch { .. } => false,
            Self::InsertValue { .. } => false,
            Self::Ret { .. } => false,
            Self::RetNull => false,
            Self::Switch { .. } => false,
            Self::Store { .. } => false,
            Self::StoreAtomic { .. } => false,
            Self::StoreIndexed { .. } => false,
            Self::UncondBr { .. } => false,
            Self::Unreachable => false,

            Self::Call {
                func_label,
                args: _,
                pure: _,
                no_return: _,
                fast_calling_conv: _,
            } => module.functions[func_label.id].return_type.is_some(),

            _ => true,
        }
    }

    pub fn get_output_type(&self, module: &mut Module) -> Option<ValueType> {
        match self {
            Self::Alloc { size: _, val_type } => {
                Some(ValueType::Pointer(Box::new(val_type.clone())))
            }
            Self::AllocUntyped { .. } => Some(ValueType::new_any_pointer()),

            Self::Alloca { size: _, val_type } => {
                Some(ValueType::Reference(Box::new(val_type.clone())))
            }
            Self::AllocaUntyped { .. } => Some(ValueType::new_any_pointer()),

            Self::BitCast { src: _, into } => Some(into.clone()),
            Self::Call {
                func_label,
                args: _,
                pure: _,
                no_return: _,
                fast_calling_conv: _,
            } => module.functions[func_label.id].return_type.clone(),

            Self::GrabArgument { index } => {
                let curr_func = module.pos_function.as_ref().unwrap();

                Some(module.functions[curr_func.id].arguments[*index].clone())
            }

            Self::Not { .. } => Some(ValueType::Int(false, 1)),
            Self::CompareOperationFloat { .. } => Some(ValueType::Int(false, 1)),
            Self::CompareOperationInt { .. } => Some(ValueType::Int(false, 1)),
            Self::ConstFloat { val: _, size } => Some(ValueType::Float(*size)),
            Self::ConstInt {
                val: _,
                size,
                signed,
            } => Some(ValueType::Int(*signed, *size)),
            Self::ConstString { .. } => Some(ValueType::new_any_pointer()),
            Self::ConstPointer { .. } => Some(ValueType::new_any_pointer()),
            Self::ConstStruct { ty, values: _ } => Some(ty.clone()),
            Self::ConstArray { values } => Some(ValueType::new_array(values[0].value_type.clone())),
            Self::ConstArraySame { value, count: _ } => {
                Some(ValueType::new_array(value.value_type.clone()))
            }

            Self::Copy { val } => Some(val.value_type.clone()),
            Self::ExtractValue { struct_val, index } => Some(struct_val.fields[*index].clone()),
            Self::FloatExtend { val: _, into } => Some(into.clone()),
            Self::FloatToInt { val: _, into } => Some(into.clone()),
            Self::FloatTruncate { val: _, into } => Some(into.clone()),
            Self::Gep { base, offset: _ } => Some(base.base.value_type.clone()),
            Self::IntExtend { val: _, into } => Some(into.clone()),
            Self::IntToFloat { val: _, into } => Some(into.clone()),
            Self::IntTruncate { val: _, into } => Some(into.clone()),
            Self::Load { source } => Some(source.inner_type.clone()),
            Self::LoadAtomic {
                source,
                ordering: _,
            } => Some(source.inner_type.clone()),
            Self::LoadIndexed { base, index: _ } => Some(base.inner_type.clone()),
            Self::MathOperationFloat {
                a,
                b: _,
                op: _,
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

            Self::ConstFloat { val, size: _ } => ConstantData::Float(*val),
            Self::ConstPointer { addr } => ConstantData::Pointer(*addr),

            _ => ConstantData::None,
        }
    }
}
