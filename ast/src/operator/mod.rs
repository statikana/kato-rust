/// an operator written between two operands, e.g. `lhs + rhs`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}

/// an operator written before its operand, e.g. `-subject`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate,
}

impl BinaryOperator {
    /// used when printing, e.g. `Add`
    pub fn name(&self) -> &'static str {
        match self {
            BinaryOperator::Add => "Add",
            BinaryOperator::Sub => "Sub",
            BinaryOperator::Mul => "Mul",
            BinaryOperator::Div => "Div",
        }
    }

    /// how it's written in source, e.g. `+`
    pub fn symbol(&self) -> &'static str {
        match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Sub => "-",
            BinaryOperator::Mul => "*",
            BinaryOperator::Div => "/",
        }
    }
}

impl UnaryOperator {
    /// used when printing, e.g. `Negate`
    pub fn name(&self) -> &'static str {
        match self {
            UnaryOperator::Negate => "Negate",
        }
    }

    /// how it's written in source, e.g. `-`
    pub fn symbol(&self) -> &'static str {
        match self {
            UnaryOperator::Negate => "-",
        }
    }
}
