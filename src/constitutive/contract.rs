use eunomia::RealField;

use crate::ThermophysicalProperties;

/// Static constitutive law mapping a material state to properties.
///
/// The generic associated state family permits implementations to borrow
/// temperature, pressure, phase fraction, or solver state without allocation.
/// Implementations and consumers remain statically dispatched.
#[diagnostic::on_unimplemented(
    message = "this type does not implement a Proteus constitutive law",
    note = "implement ConstitutiveLaw<T> and define its borrowed State<'a> family"
)]
pub trait ConstitutiveLaw<T: RealField> {
    /// State view consumed by one evaluation.
    type State<'a>
    where
        Self: 'a,
        T: 'a;

    /// Evaluation failure.
    type Error;

    /// Evaluate properties at `state`.
    ///
    /// # Errors
    ///
    /// Returns the implementation's typed failure when the state lies outside
    /// the law's domain.
    fn properties<'a>(
        &'a self,
        state: Self::State<'a>,
    ) -> Result<ThermophysicalProperties<T>, Self::Error>
    where
        T: 'a;
}
