//! Definitions for the int values in the MIR

use crate::values::{BaseSSAValue, SSAValueLike, ValueType, consts::ConstantData};

/// A variant of [`BaseSSAValue`] that is an integer
#[derive(Clone)]
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

    pub fn enforces_boolean(&self) -> Result<(), ()> {
        if self.size != 1 {
            return Err(());
        }

        Ok(())
    }
}

impl TryFrom<BaseSSAValue> for SSAIntValue {
    type Error = ();

    fn try_from(value: BaseSSAValue) -> Result<Self, Self::Error> {
        if let ValueType::Int(signed, size) = (&value).value_type {
            Ok(Self {
                base: value,
                signed: signed,
                size: size,
            })
        } else {
            Err(())
        }
    }
}

impl Into<BaseSSAValue> for SSAIntValue {
    fn into(self) -> BaseSSAValue {
        self.base.clone()
    }
}

impl SSAValueLike for SSAIntValue {
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
