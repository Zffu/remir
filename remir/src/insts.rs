//! Definitions of the instruction set of Remir

use crate::values::{BaseSSAValue, int::SSAIntValue, ptr::SSAPointerValue};

/// Represents an instruction in the MIR.
pub enum Instruction {
    // Constant instructions
    ConstInt {
        val: i128,
        size: usize,
        signed: bool,
    },

    ConstFloat {
        val: f64,
        size: usize,
        signed: bool,
    },

    ConstPointer {
        addr: usize,
    },

    // Memory / register instructions
    Copy {
        val: BaseSSAValue,
    },

    AllocConst {
        size: usize,
    },

    Alloc {
        size: SSAIntValue,
    },

    AllocaConst {
        size: usize,
    },

    Alloca {
        size: SSAIntValue,
    },

    Free {
        ptr: SSAPointerValue,
    },
}
