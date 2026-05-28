use crate::values::{BaseSSAValue, ValueType};

pub struct SSAFloatValue {
    pub base: BaseSSAValue,
    pub signed: bool,
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
