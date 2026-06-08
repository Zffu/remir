//! Definitions for value types inside of the MIR

/// The type of the SSA MIR values.
#[derive(Clone, PartialEq, Hash, Eq)]
pub enum ValueType {
    /// The float value type. Contains the signed state and size state
    Float(usize),

    /// The int value type. Contains the signed state and size state
    Int(bool, usize),

    /// The structure value type. Contains the fields
    Struct(Vec<Box<ValueType>>),

    /// The pointer value type.
    Pointer(Box<ValueType>),

    /// A non null pointer value type
    Reference(Box<ValueType>),

    /// An array of values
    Array(Box<ValueType>),

    /// The unkown value type
    Unknown,
}

impl ValueType {
    /// Creates a new float type with the given size
    #[inline(always)]
    pub fn new_float(size: usize) -> Self {
        Self::Float(size)
    }

    /// Creates a new int type with the given signed state and the given size
    #[inline(always)]
    pub fn new_int(signed: bool, size: usize) -> Self {
        Self::Int(signed, size)
    }

    /// Creates a new struct type with the given field types
    #[inline(always)]
    pub fn new_struct(fields: Vec<ValueType>) -> Self {
        Self::Struct(fields.iter().map(|f| Box::new(f.clone())).collect())
    }

    /// Creates a new pointer type with the given inner type
    #[inline(always)]
    pub fn new_pointer(inner_ty: ValueType) -> Self {
        Self::Pointer(Box::new(inner_ty))
    }

    /// Creates a new array type with the given inner type
    #[inline(always)]
    pub fn new_array(inner_ty: ValueType) -> Self {
        Self::Array(Box::new(inner_ty))
    }

    /// Creates a new pointer type that contains an unknown and thus represents a pointer address rather than a real pointer.
    ///
    /// Any pointers cannot be used on store and loads as they will just use unknown values
    #[inline(always)]
    pub fn new_any_pointer() -> Self {
        Self::new_pointer(ValueType::Unknown)
    }
}
