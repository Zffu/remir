//! Misc definitions

#[derive(Clone)]
pub enum MemoryOrder {
    Relaxed,
    Consume,
    Acquire,
    Release,
    AcqRel,
    SeqCst,
}

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

#[derive(Clone)]
pub enum CompareOperator {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
