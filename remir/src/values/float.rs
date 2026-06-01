//! Definitions for float values in the MIR

use crate::values::{BaseSSAValue, ValueType};

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
    type Error = ();

    fn try_from(value: BaseSSAValue) -> Result<Self, Self::Error> {
        if let ValueType::Float(size) = (&value).value_type {
            Ok(Self {
                base: value,
                size: size,
            })
        } else {
            Err(())
        }
    }
}

impl Into<BaseSSAValue> for SSAFloatValue {
    fn into(self) -> BaseSSAValue {
        self.base.clone()
    }
}
