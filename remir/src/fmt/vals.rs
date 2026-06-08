//! Formatting for values

use std::fmt::Display;

use crate::values::{
    BaseSSAValue, SSAValueLike, consts::ConstantData, float::SSAFloatValue, int::SSAIntValue,
    ptr::SSAPointerValue, structs::SSAStructValue,
};

impl Display for ConstantData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::None => "??",
            Self::Float(val) => &format!("{}", val),
            Self::Int(val) => &format!("{}", val),
            Self::Pointer(val) => &format!("*{}", val),
        };

        write!(f, "{}", res)
    }
}

macro_rules! decl_value_display {
    ($type: ty) => {
        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let const_data = self.get_constant_data();
                let display_const_data = const_data.is_something();

                write!(f, "{} ", self.get_type())?;

                if display_const_data {
                    write!(f, "{} (", const_data)?;
                }

                write!(f, "#{}", self.get_inst_index())?;

                if display_const_data {
                    write!(f, ")")?;
                }

                Ok(())
            }
        }
    };
}

decl_value_display!(BaseSSAValue);
decl_value_display!(SSAIntValue);
decl_value_display!(SSAFloatValue);
decl_value_display!(SSAPointerValue);
decl_value_display!(SSAStructValue);
