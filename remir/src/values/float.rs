//! Definitions for float values in the MIR

use crate::values::{BaseSSAValue, ValueType};

/// A variant of [`BaseSSAValue`] that is a float
#[derive(Clone, Copy)]
pub struct SSAFloatValue {
    /// The base of the value
    pub base: BaseSSAValue,

    /// The signed state
    pub signed: bool,

    /// The size state
    pub size: usize,
}

impl SSAFloatValue {
    /// Creates a new [`SSAFloatValue`]
    #[inline(always)]
    pub fn new(inst_ind: usize, signed: bool, size: usize) -> Self {
        Self {
            base: BaseSSAValue::new(inst_ind, ValueType::Float(signed, size)),
            signed,
            size,
        }
    }
}

impl TryFrom<BaseSSAValue> for SSAFloatValue {
    type Error = &'static str;

    fn try_from(value: BaseSSAValue) -> Result<Self, Self::Error> {
        if let ValueType::Float(signed, size) = (&value).value_type {
            Ok(Self {
                base: value,
                signed: signed,
                size: size,
            })
        } else {
            Err("BaseSSAValue is not of type float")
        }
    }
}

impl Into<BaseSSAValue> for SSAFloatValue {
    fn into(self) -> BaseSSAValue {
        self.base.clone()
    }
}
