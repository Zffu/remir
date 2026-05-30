//! Functions used to build instructions and directly append them into the module.

pub mod cmp;
pub mod consts;
pub mod math;
pub mod regs;

pub use cmp::*;
pub use consts::*;
pub use math::*;
pub use regs::*;
