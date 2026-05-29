//! Definitions for the int values in the MIR

use crate::values::{BaseSSAValue, ValueType};

/// A variant of [`BaseSSAValue`] that is an integer
#[derive(Clone, Copy)]
pub struct SSAIntValue {
    /// The base of the value
    pub base: BaseSSAValue,

    /// The signed state
    pub signed: bool,

    /// The size state
    pub size: usize,
}

impl SSAIntValue {
    /// Creates a new [`SSAIntValue`]
    #[inline(always)]
    pub fn new(inst_ind: usize, signed: bool, size: usize) -> Self {
        Self {
            base: BaseSSAValue::new(inst_ind, ValueType::Int(signed, size)),
            signed,
            size,
        }
    }
}

impl TryFrom<BaseSSAValue> for SSAIntValue {
    type Error = &'static str;

    fn try_from(value: BaseSSAValue) -> Result<Self, Self::Error> {
        if let ValueType::Int(signed, size) = (&value).value_type {
            Ok(Self {
                base: value,
                signed: signed,
                size: size,
            })
        } else {
            Err("BaseSSAValue is not of type int")
        }
    }
}

impl Into<BaseSSAValue> for SSAIntValue {
    fn into(self) -> BaseSSAValue {
        self.base.clone()
    }
}
