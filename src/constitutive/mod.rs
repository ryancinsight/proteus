//! Constitutive-law seams.

mod constant;
mod contract;

pub use constant::{ConstantLaw, NoState};
pub use contract::ConstitutiveLaw;
