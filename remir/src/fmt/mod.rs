//! Formatting module for every element inside of Remir. Allows Remir to generate IR files

use std::fmt::Display;

use crate::values::ValueType;

pub mod vals;

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Int(signed, size) => {
                if *signed {
                    format!("s{}", size)
                } else {
                    format!("u{}", size)
                }
            }

            Self::Float(size) => {
                format!("f{}", size)
            }

            Self::Struct(_) => {
                format!("struct")
            }

            Self::Pointer(inner) => {
                format!("ptr({})", inner)
            }

            Self::Unknown => "??".to_string(),
        };

        write!(f, "{}", res)
    }
}
