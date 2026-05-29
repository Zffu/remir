//! Definitions for values in the Remir MIR

pub mod float;
pub mod int;
pub mod ptr;

/// The type of the SSA MIR values.
#[derive(Clone, Copy, PartialEq)]
pub enum ValueType {
    /// The float value type. Contains the signed state and size state
    Float(bool, usize),

    /// The int value type. Contains the signed state and size state
    Int(bool, usize),

    /// The pointer value type.
    Pointer,
}

/// Represents a base SSA value inside of the IR.
/// An SSA value is a value potentially obtained from using an instruction.
#[derive(Clone, Copy)]
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
