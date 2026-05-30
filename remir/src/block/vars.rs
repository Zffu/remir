//! Definitions for block variables

use crate::{
    builders::{
        atomic::{build_load_atomic, build_store_atomic},
        build_load, build_store,
    },
    module::Module,
    utils::atomic::MemoryOrder,
    values::{BaseSSAValue, ptr::SSAPointerValue},
};

/// Helper to use variables inside of Remir.
/// Allows to automatically handle things like atomic state, pointer writing and more.
#[derive(Clone)]
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

    pub fn write(&mut self, module: &mut Module, val: BaseSSAValue) -> Result<(), ()> {
        if !self.write_as_pointer {
            self.held_value = Some(val);
            return Ok(());
        }

        let ptr: SSAPointerValue = unsafe {
            self.held_value
                .clone()
                .unwrap_unchecked()
                .try_into()
                .unwrap_unchecked()
        };

        if self.atomic_state.is_none() {
            return build_store(module, ptr, val);
        }

        let atomic_state = unsafe { self.atomic_state.clone().unwrap_unchecked() };

        return build_store_atomic(module, ptr, val, atomic_state);
    }

    pub fn read(&self, module: &mut Module) -> Result<BaseSSAValue, ()> {
        if !self.write_as_pointer {
            return match &self.held_value {
                Some(v) => Ok(v.clone()),
                None => Err(()),
            };
        }

        let ptr: SSAPointerValue = unsafe {
            self.held_value
                .clone()
                .unwrap_unchecked()
                .try_into()
                .unwrap_unchecked()
        };

        if self.atomic_state.is_none() {
            return build_load(module, ptr);
        }

        let atomic_state = unsafe { self.atomic_state.clone().unwrap_unchecked() };

        return build_load_atomic(module, ptr, atomic_state);
    }
}
