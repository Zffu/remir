//! Definitions for the pointer values in the MIR

use crate::values::{BaseSSAValue, SSAValueLike, ValueType, consts::ConstantData};

/// A variant of [`BaseSSAValue`] that is a pointer.
#[derive(Clone)]
pub struct SSAPointerValue {
    /// The base of the value
    pub base: BaseSSAValue,

    pub inner_type: ValueType,
}

impl SSAPointerValue {
    /// Creates a new [`SSAPointerValue`]
    #[inline(always)]
    pub fn new(inst_ind: usize, inner_type: ValueType) -> Self {
        Self {
            inner_type: inner_type.clone(),
            base: BaseSSAValue::new(inst_ind, ValueType::Pointer(Box::new(inner_type))),
        }
    }
}

impl TryFrom<BaseSSAValue> for SSAPointerValue {
    type Error = ();

    fn try_from(value: BaseSSAValue) -> Result<Self, Self::Error> {
        if let ValueType::Pointer(inner) = (&value).value_type.clone() {
            Ok(Self {
                base: value,
                inner_type: *inner,
            })
        } else {
            Err(())
        }
    }
}

impl Into<BaseSSAValue> for SSAPointerValue {
    fn into(self) -> BaseSSAValue {
        self.base.clone()
    }
}

impl SSAValueLike for SSAPointerValue {
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
