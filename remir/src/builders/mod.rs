//! Functions used to build instructions and directly append them into the module.

pub mod branches;
pub mod cmp;
pub mod consts;
pub mod math;
pub mod regs;

pub use branches::*;
pub use cmp::*;
pub use consts::*;
pub use math::*;
pub use regs::*;
