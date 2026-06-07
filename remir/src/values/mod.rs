//! Definitions for values in the Remir MIR

use crate::values::consts::ConstantData;

pub mod array;
pub mod consts;
pub mod float;
pub mod int;
pub mod ptr;
pub mod structs;
pub mod types;

pub use types::ValueType;

/// Represents a base SSA value inside of the IR.
/// An SSA value is a value potentially obtained from using an instruction.
#[derive(Clone)]
pub struct BaseSSAValue {
    /// The index responsible to the creation of said value
    pub inst_ind: usize,

    /// The type of value
    pub value_type: ValueType,

    pub constant: ConstantData,
}

pub trait SSAValueLike {
    fn get_type(&self) -> ValueType;
    fn get_constant_data(&self) -> ConstantData;
    fn get_inst_index(&self) -> usize;
}

impl BaseSSAValue {
    /// Creates a new [`BaseSSAValue`] based on the given type and the given instruction index
    pub fn new(inst_ind: usize, value_type: ValueType) -> Self {
        Self {
            inst_ind,
            value_type,
            constant: ConstantData::None,
        }
    }

    /// Creates a new [`BaseSSAValue`] based on the given type and the given instruction index. This will hold constant data
    pub fn new_const(inst_ind: usize, value_type: ValueType, constant: ConstantData) -> Self {
        Self {
            inst_ind,
            value_type,
            constant,
        }
    }
}

impl SSAValueLike for BaseSSAValue {
    fn get_type(&self) -> ValueType {
        self.value_type.clone()
    }

    fn get_constant_data(&self) -> ConstantData {
        self.constant.clone()
    }

    fn get_inst_index(&self) -> usize {
        self.inst_ind
    }
}
