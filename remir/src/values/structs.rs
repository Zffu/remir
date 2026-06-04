//! Definitions for the structure values in the MIR

use crate::{
    errs::RemirReturnableError,
    return_err,
    values::{BaseSSAValue, SSAValueLike, ValueType, consts::ConstantData},
};

/// A variant of [`BaseSSAValue`] that is a structure
#[derive(Clone)]
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
    type Error = RemirReturnableError;

    fn try_from(value: BaseSSAValue) -> Result<Self, Self::Error> {
        if let ValueType::Struct(fields) = (&value).value_type.clone() {
            Ok(Self {
                base: value,
                fields: fields.iter().map(|f| *f.clone()).collect(),
            })
        } else {
            return_err!("Tried casting a non struct value into a struct");
        }
    }
}

impl Into<BaseSSAValue> for SSAStructValue {
    fn into(self) -> BaseSSAValue {
        self.base.clone()
    }
}

impl SSAValueLike for SSAStructValue {
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
