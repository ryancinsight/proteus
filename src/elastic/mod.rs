//! Isotropic linear-elastic material state.
//!
//! Proteus owns the `(E, nu) <-> (lambda, mu) <-> (c_p, c_s)` conversion
//! contract and the named isotropic-solid catalog. Consumers hold elastic
//! *state* through [`IsotropicModuli`] and keep their own kinematics, balance
//! operators, and discretization.

mod catalog;
mod error;
mod moduli;

pub use catalog::{IsotropicSolid, NamedIsotropicSolid};
pub use error::{ElasticConstraint, ElasticQuantity, InvalidElasticModuli};
pub use moduli::IsotropicModuli;
