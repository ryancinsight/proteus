//! Validated material-property boundaries.

/// Declare a newtype over a validated Aequitas quantity.
///
/// The generated type is `#[repr(transparent)]` over the supplied quantity, so
/// it shares that quantity's size and alignment. The `new` constructor runs the
/// named validator on the canonical-SI scalar and wraps the validated value back
/// into the same quantity, and the accessors expose the quantity without
/// conversion.
///
/// The parameters are, in order, the newtype name, the quantity it wraps, the
/// [`PropertyKind`] the validator reports, and the validator to run.
macro_rules! validated_quantity_newtype {
    (
        $(#[$doc:meta])*
        $name:ident,
        $quantity:ty,
        $kind:expr,
        $validator:path $(,)?
    ) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $name<T>($quantity);

        impl<T: RealField> $name<T> {
            /// Validate a dimensional quantity and wrap it in this newtype.
            ///
            /// # Errors
            ///
            /// Returns [`InvalidProperty`] when the canonical-SI value violates
            /// this property's constraint.
            pub fn new(value: $quantity) -> Result<Self, InvalidProperty<T>> {
                $validator($kind, value.into_base())
                    .map(|valid| Self(<$quantity>::from_base(valid)))
            }
        }

        impl<T> $name<T> {
            /// Borrow the Aequitas quantity without conversion or copying.
            #[must_use]
            pub const fn quantity(&self) -> &$quantity {
                &self.0
            }

            /// Move out the Aequitas quantity.
            #[must_use]
            pub fn into_quantity(self) -> $quantity {
                self.0
            }
        }
    };
}

mod conductivity;
mod density;
mod error;
mod heat_capacity;
mod validation;

pub use conductivity::ThermalConductivity;
pub use density::MassDensity;
pub use error::{InvalidProperty, PropertyConstraint, PropertyKind};
pub use heat_capacity::SpecificHeatCapacity;
