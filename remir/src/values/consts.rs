//! Definitions for constant data structures in the MIR

/// Represents a constant data container.
/// Holds the constant data value
#[derive(Debug, Clone, Copy)]
pub enum ConstantData {
    /// An integer constant data
    Int(i128),

    /// A float constant data
    Float(f64),

    /// A pointer constant data
    Pointer(usize),

    /// No constant data
    None,
}

impl ConstantData {
    /// Converts the [`ConstantData`] object into an int.
    ///
    /// # Panics
    /// This function will panic if the [`ConstantData`] is not of [`ConstantData::Int`].
    ///
    pub fn as_int(&self) -> i128 {
        match self {
            Self::Int(v) => *v,
            _ => panic!("Expected Self::Int got {:#?}", self),
        }
    }

    /// Converts the [`ConstantData`] object into a float.
    ///
    /// # Panics
    /// This function will panic if the [`ConstantData`] is not of [`ConstantData::Float`].
    ///
    pub fn as_float(self) -> f64 {
        match self {
            Self::Float(v) => v,
            _ => panic!("Expected Self::Float got {:#?}", self),
        }
    }

    /// Converts the [`ConstantData`] object into a float.
    ///
    /// # Panics
    /// This function will panic if the [`ConstantData`] is not of [`ConstantData::Float`].
    ///
    pub fn as_ptr(self) -> usize {
        match self {
            Self::Pointer(v) => v,
            _ => panic!("Expected Self::Pointer got {:#?}", self),
        }
    }

    /// Checks whether the [`ConstantData`] actually holds any constant data of any kind.
    pub fn is_something(&self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }
}
