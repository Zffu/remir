//! Definitions for float values in the MIR

use crate::{
    errs::RemirReturnableError,
    return_err,
    values::{BaseSSAValue, SSAValueLike, ValueType, consts::ConstantData},
};

/// A variant of [`BaseSSAValue`] that is a float
#[derive(Clone)]
pub struct SSAFloatValue {
    /// The base of the value
    pub base: BaseSSAValue,

    /// The size state
    pub size: usize,
}

impl SSAFloatValue {
    /// Creates a new [`SSAFloatValue`]
    #[inline(always)]
    pub fn new(inst_ind: usize, size: usize) -> Self {
        Self {
            base: BaseSSAValue::new(inst_ind, ValueType::Float(size)),
            size,
        }
    }
}

impl TryFrom<BaseSSAValue> for SSAFloatValue {
    type Error = RemirReturnableError;

    fn try_from(value: BaseSSAValue) -> Result<Self, Self::Error> {
        if let ValueType::Float(size) = (&value).value_type {
            Ok(Self {
                base: value,
                size: size,
            })
        } else {
            return_err!("Tried casting a non float value into a float")
        }
    }
}

impl Into<BaseSSAValue> for SSAFloatValue {
    fn into(self) -> BaseSSAValue {
        self.base.clone()
    }
}

impl SSAValueLike for SSAFloatValue {
    fn get_type(&self) -> ValueType {
        self.base.get_type()
    }

    fn get_constant_data(&self) -> ConstantData {
        self.base.get_constant_data()
    }

    fn get_inst_index(&self) -> usize {
        self.base.get_inst_index()
    }
}
