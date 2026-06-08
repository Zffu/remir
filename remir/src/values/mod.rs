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

    /// The constant data of the value. Holds potentially the constant value
    pub constant: ConstantData,
}

/// Represents an SSA value with functions to obtain information about the SSA value.
pub trait SSAValueLike {
    /// Returns the value's type as an [`ValueType`]
    fn get_type(&self) -> ValueType;

    /// Returns the constant data information of the value.
    fn get_constant_data(&self) -> ConstantData;

    /// Returns the instruction index corresponding to the value's creation.
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
