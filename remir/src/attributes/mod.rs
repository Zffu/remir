//! Definitions of attributes for Remir objects. These attributes can help with optimization and are automatically added in the attribute pass.
//!

/// Attributes that can be added to a function as whole
pub enum FunctionAttributes {
    /// The function doesn't return and thus ends the program
    NoReturn,

    /// The function rarely gets exectued
    Cold,

    /// The function frequently gets executed
    Hot,

    /// No side effects (doesn't modify any outside memory) except reading memory.
    Pure,

    /// No side effects and doesn't read outside memory (only argument manipulation is allowed)
    Const,

    /// Returns a fresh pointer that doesn't exist inside of the function arguments.
    FreshPointer,

    /// Never returns a null pointer
    ReturnsNonNull,

    /// Doesn't call another function besides itself
    NoSideCalls,

    /// This function only returns recursion results or arguments
    Recursion,
}

/// Attributes that can be added to variables
pub enum VariableAttributes {
    /// A variable that is unused
    /// This attribute should not be inherited
    Unused,

    /// Represents a variable that is an argument
    /// This attribute should not be inherited
    Argument,

    /// Represents a variable that originates from a recursive call
    /// This attribute should be inherited if all instruction members
    /// share this attribute and if the instruction allows it (eg: math operation)
    Recursive,

    /// Represents a variable that can be resolved at compile time
    /// This attribute should be inherited if all instruction members
    /// share this attribute and if the instruction allows it (eg: math operation)
    Comptime,

    /// The variable originates from a static variable.
    /// This attribute should be inherited if at least one instruction member
    /// has this attribute
    StaticVar,

    /// This variable escapes the function (eg: from a function call or a return)
    /// This attribute should not be inherited
    Escapes,

    /// Represents a pointer / reference that isn't represented anywhere else and doesn't overlap with any pointer.
    /// NoAlias is allowed to be given when:
    /// - The pointer isn't from a static variable
    /// - The pointer isn't passed inside of a function argument
    /// - The
    /// This attribute should not be inherited
    NoAlias,
}
