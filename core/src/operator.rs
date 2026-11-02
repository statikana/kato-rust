use std::fmt;

use crate::types::Type;

mod add;
mod sub;
mod mul;
mod div;
mod negate;

pub use add::Add;
pub use sub::Sub;
pub use mul::Mul;
pub use div::Div;
pub use negate::Negate;

/// an operator written between two operands, e.g. `lhs + rhs`
pub trait BinaryOperator {
    /// used when printing, e.g. `Add`
    fn name(&self) -> &'static str;
    /// how it's written in source, e.g. `+`
    fn symbol(&self) -> &'static str;
    /// the type of `lhs op rhs`. `None` means the operand types don't support it
    fn result_type(&self, lhs: &Type, rhs: &Type) -> Option<Type>;
}

/// an operator written before its operand, e.g. `-subject`
pub trait UnaryOperator {
    /// used when printing, e.g. `Negate`
    fn name(&self) -> &'static str;
    /// how it's written in source, e.g. `-`
    fn symbol(&self) -> &'static str;
    /// the type of `op subject`. `None` means the subject's type doesn't support it
    fn result_type(&self, subject: &Type) -> Option<Type>;
}

impl fmt::Debug for dyn BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl fmt::Debug for dyn UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
