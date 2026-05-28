use crate::values::{BaseSSAValue, ValueType};

/// A variant of [`BaseSSAValue`] that is a pointer.
#[derive(Clone, Copy)]
pub struct SSAPointerValue {
    pub base: BaseSSAValue,
}

impl SSAPointerValue {
    /// Creates a new [`SSAPointerValue`]
    #[inline(always)]
    pub fn new(inst_ind: usize) -> Self {
        Self {
            base: BaseSSAValue::new(inst_ind, ValueType::Pointer),
        }
    }
}

impl TryFrom<BaseSSAValue> for SSAPointerValue {
    type Error = &'static str;

    fn try_from(value: BaseSSAValue) -> Result<Self, Self::Error> {
        if let ValueType::Pointer = (&value).value_type {
            Ok(Self { base: value })
        } else {
            Err("BaseSSAValue is not of type pointer")
        }
    }
}

impl Into<BaseSSAValue> for SSAPointerValue {
    fn into(self) -> BaseSSAValue {
        self.base.clone()
    }
}
