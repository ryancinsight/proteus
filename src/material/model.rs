use alloc::{borrow::Cow, string::String};

use eunomia::RealField;

use crate::{ConstitutiveLaw, ThermophysicalProperties};

/// Named material composed with one statically dispatched constitutive law.
#[derive(Clone, Debug, PartialEq)]
pub struct Material<'name, Law> {
    name: Cow<'name, str>,
    law: Law,
}

impl<'name, Law> Material<'name, Law> {
    /// Construct with a borrowed name and no name allocation.
    #[must_use]
    pub const fn borrowed(name: &'name str, law: Law) -> Self {
        Self {
            name: Cow::Borrowed(name),
            law,
        }
    }

    /// Construct with an owned runtime name.
    #[must_use]
    pub fn owned(name: String, law: Law) -> Self {
        Self {
            name: Cow::Owned(name),
            law,
        }
    }

    /// Borrow the material name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Borrow the constitutive law.
    #[must_use]
    pub const fn law(&self) -> &Law {
        &self.law
    }

    /// Evaluate the material's constitutive law.
    ///
    /// # Errors
    ///
    /// Returns the law's typed evaluation failure.
    pub fn properties<'a, T>(
        &'a self,
        state: Law::State<'a>,
    ) -> Result<ThermophysicalProperties<T>, Law::Error>
    where
        T: RealField + 'a,
        Law: ConstitutiveLaw<T>,
    {
        self.law.properties(state)
    }
}
