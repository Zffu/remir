//! Definitions of the instruction set of Remir

/// Represents an instruction in the MIR.
pub enum Instruction {
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
}
