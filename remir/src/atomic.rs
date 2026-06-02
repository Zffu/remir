//! Atomic-related utilities

#[derive(Clone)]
pub enum MemoryOrder {
    Relaxed,
    Consume,
    Acquire,
    Release,
    AcqRel,
    SeqCst,
}
