//! Definitions for the array values in the MIR

use crate::{
    errs::RemirReturnableError,
    return_err,
    values::{BaseSSAValue, SSAValueLike, ValueType},
};

/// A variant of [`BaseSSAValue`] that is an array
pub struct SSAArrayValue {
    /// The base of the SSA value
    pub base: BaseSSAValue,

    /// The inner type of the array. Represents the type of values contained inside of the array
    pub inner_type: ValueType,
}

impl SSAArrayValue {
    /// Creates a new [`SSAArrayValue`]
    pub fn new(inst_ind: usize, inner_type: ValueType) -> Self {
        Self {
            base: BaseSSAValue::new(inst_ind, ValueType::Array(Box::new(inner_type.clone()))),
            inner_type,
        }
    }
}

impl TryFrom<BaseSSAValue> for SSAArrayValue {
    type Error = RemirReturnableError;

    fn try_from(value: BaseSSAValue) -> Result<Self, Self::Error> {
        if let ValueType::Array(inner) = (&value).value_type.clone() {
            return Ok(Self {
                base: value,
                inner_type: *inner,
            });
        }

        return_err!("Tried casting a non array value into a pointzer")
    }
}

impl Into<BaseSSAValue> for SSAArrayValue {
    fn into(self) -> BaseSSAValue {
        self.base
    }
}

impl SSAValueLike for SSAArrayValue {
    fn get_type(&self) -> ValueType {
        self.base.get_type()
    }

    fn get_constant_data(&self) -> super::consts::ConstantData {
        self.base.get_constant_data()
    }

    fn get_inst_index(&self) -> usize {
        self.base.get_inst_index()
    }
}
