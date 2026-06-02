//! Misc definitions

/// The order of memory for atomic operations.
/// Also named `AtomicOrder`
#[derive(Clone)]
pub enum MemoryOrder {
    Relaxed,
    Consume,
    Acquire,
    Release,
    AcqRel,
    SeqCst,
}

/// The operators for math operations and instructions
#[derive(Clone)]
pub enum MathOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

/// The operators for compare instructions
#[derive(Clone)]
pub enum CompareOperator {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
