//! Definitions for values in the Remir MIR

pub mod float;
pub mod int;

/// The type of the SSA MIR values.
#[derive(Clone, PartialEq)]
pub enum ValueType {
    Float(bool, usize),
    Int(bool, usize),
    Pointer,
}

#[derive(Clone)]
pub struct BaseSSAValue {
    /// The index responsible to the creation of said value
    pub inst_ind: usize,

    /// The type of value
    pub value_type: ValueType,
}

impl BaseSSAValue {
    /// Creates a new [`BaseSSAValue`] based on the given type and the given instruction index
    pub fn new(inst_ind: usize, value_type: ValueType) -> Self {
        Self {
            inst_ind,
            value_type,
        }
    }
}
