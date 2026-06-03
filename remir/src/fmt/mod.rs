//! Formatting module for every element inside of Remir. Allows Remir to generate IR files

use std::fmt::Display;

use crate::{
    block::BlockReference,
    func::FunctionReference,
    misc::{CompareOperator, MathOperator, MemoryOrder},
    values::ValueType,
};

pub mod blocks;
pub mod insts;
pub mod utils;
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

impl Display for MemoryOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Relaxed => "relaxed",
            Self::Consume => "consume",
            Self::Acquire => "acquire",
            Self::Release => "release",
            Self::AcqRel => "acqrel",
            Self::SeqCst => "seqcst",
        };

        write!(f, "{}", res)
    }
}

impl Display for MathOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Mul => "mul",
            Self::Div => "div",
            Self::Mod => "mod",
            Self::And => "and",
            Self::Or => "or",
            Self::Xor => "xor",
            Self::Shl => "shl",
            Self::Shr => "shr",
        };

        write!(f, "{}", res)
    }
}

impl Display for CompareOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Eq => "eq",
            Self::Ne => "ne",
            Self::Lt => "lt",
            Self::Le => "le",
            Self::Gt => "gt",
            Self::Ge => "ge",
        };

        write!(f, "{}", res)
    }
}

impl Display for BlockReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}_{}", self.name, self.id)
    }
}

impl Display for FunctionReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.name)
    }
}
