use std::fmt;

use crate::nodes::NodeBehavior;

pub struct ASTInteger(pub i128);

impl NodeBehavior for ASTInteger {
    fn name(&self) -> &'static str {
        "Integer"
    }

    fn fmt_labeled(
        &self,
        label: &str,
        _children: &[&dyn fmt::Debug],
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_tuple(label).field(&self.0).finish()
    }
}
