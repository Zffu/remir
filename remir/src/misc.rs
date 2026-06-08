//! Misc definitions

/// The order of memory for atomic operations.
/// Also named `AtomicOrder`
#[derive(Clone)]
pub enum MemoryOrder {
    /// No ordering guarantees besides atomicity
    /// - Only guarantees the read/write itself is atomic
    /// - No sync with other threads
    /// - Can be freely reordered
    Relaxed,

    /// Weaker version of [`MemoryOrder::Acquire`]
    /// - Only enforces orering along data deps
    /// - If load with consume, dependants cannot load before it.
    Consume,

    /// Prevents later memory operations from moving before the load
    /// - If you read something with acquire, all followings read/writes will happen after it.
    /// - Syncs with a release store from another thread
    Acquire,

    /// Prevents earlier memory operations from moving after the store
    /// - Ensures all writes in this thread happen before the release store
    Release,

    /// Combination of [`MemoryOrder::Acquire`] and [`MemoryOrder::Release`]
    AcqRel,

    /// Sequentially consistent
    /// - Everything behavers as if all atomic operations occur in a single global order
    /// - Enforces [`MemoryOrder::Acquire`] + [`MemoryOrder::Release`] global ordering across all threads
    SeqCst,
}

/// The operators for math operations and instructions
#[derive(Clone)]
pub enum MathOperator {
    /// The addition operator
    Add,

    /// The substraction operator
    Sub,

    /// The multiplication operator
    Mul,

    /// The division operator
    Div,

    /// The modulo operator
    Mod,

    /// The and operator
    And,

    /// The or operator
    Or,

    /// The xor operator
    Xor,

    /// The shift left operator
    Shl,

    /// The shift right operator
    Shr,

    /// The nor operator
    Nor,
}

/// The operators for compare instructions
#[derive(Clone)]
pub enum CompareOperator {
    /// The equal compare operator
    Eq,

    /// The not equal compare operator
    Ne,

    /// The lower than compare operator
    Lt,

    /// The lower than equal operator
    Le,

    /// The greater than operator
    Gt,

    /// The greater than equal operator
    Ge,
}
