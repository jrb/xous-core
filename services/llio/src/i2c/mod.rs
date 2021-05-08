#[cfg(not(target_os = "none"))]
mod hosted;
#[cfg(not(target_os = "none"))]
pub use crate::i2c::hosted::*;

#[cfg(target_os = "none")]
mod hardware;
#[cfg(target_os = "none")]
pub(crate) use crate::i2c::hardware::*;
