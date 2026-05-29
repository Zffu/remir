//! Definitions for the structure values in the MIR

use crate::values::{BaseSSAValue, ValueType};

/// A variant of [`BaseSSAValue`] that is a structure
pub struct SSAStructValue {
    pub base: BaseSSAValue,
    pub fields: Vec<ValueType>,
}

impl SSAStructValue {
    /// Creates a new [`SSAStructValue`]
    pub fn new(inst_ind: usize, fields: Vec<ValueType>) -> Self {
        let boxed_fields: Vec<Box<ValueType>> =
            fields.iter().map(|f| Box::new(f.clone())).collect();

        Self {
            fields,
            base: BaseSSAValue::new(inst_ind, ValueType::Struct(boxed_fields)),
        }
    }
}

impl TryFrom<BaseSSAValue> for SSAStructValue {
    type Error = &'static str;

    fn try_from(value: BaseSSAValue) -> Result<Self, Self::Error> {
        if let ValueType::Struct(fields) = (&value).value_type.clone() {
            Ok(Self {
                base: value,
                fields: fields.iter().map(|f| *f.clone()).collect(),
            })
        } else {
            Err("BaseSSAValue is not of type pointer")
        }
    }
}

impl Into<BaseSSAValue> for SSAStructValue {
    fn into(self) -> BaseSSAValue {
        self.base.clone()
    }
}
