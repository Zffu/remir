//! Definitions for block variables

use crate::{
    utils::atomic::MemoryOrder,
    values::{BaseSSAValue, ptr::SSAPointerValue},
};

/// Helper to use variables inside of Remir.
/// Allows to automatically handle things like atomic state, pointer writing and more.
pub struct BlockVariable {
    pub name: String,

    pub held_value: Option<BaseSSAValue>,

    pub write_as_pointer: bool,

    /// Atomic state requires a pointer
    pub atomic_state: Option<MemoryOrder>,
}

impl BlockVariable {
    pub fn new_ssa(name: String, val: Option<BaseSSAValue>) -> Self {
        Self {
            name,
            held_value: val,
            write_as_pointer: false,
            atomic_state: None,
        }
    }

    pub fn new_pointer(name: String, val: SSAPointerValue) -> Self {
        Self {
            name,
            held_value: Some(val.into()),
            write_as_pointer: true,
            atomic_state: None,
        }
    }

    pub fn new_atomic(name: String, val: SSAPointerValue, order: MemoryOrder) -> Self {
        Self {
            name,
            held_value: Some(val.into()),
            write_as_pointer: true,
            atomic_state: Some(order),
        }
    }
}
