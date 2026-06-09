//! Functions used to build instructions and directly append them into the module.

pub mod atomic;
pub mod branches;
pub mod cmp;
pub mod consts;
pub mod func;
pub mod hints;
pub mod lazy;
pub mod math;
pub mod mem;
pub mod regs;
pub mod vals;

pub use branches::*;
pub use cmp::*;
pub use consts::*;
pub use func::*;
pub use hints::*;
pub use lazy::*;
pub use math::*;
pub use mem::*;
pub use regs::*;
pub use vals::*;
