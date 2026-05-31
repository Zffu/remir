//! Functions used to build instructions and directly append them into the module.

pub mod atomic;
pub mod branches;
pub mod cmp;
pub mod consts;
pub mod func;
pub mod math;
pub mod mem;
pub mod regs;

pub use branches::*;
pub use cmp::*;
pub use consts::*;
pub use func::*;
pub use math::*;
pub use mem::*;
pub use regs::*;
