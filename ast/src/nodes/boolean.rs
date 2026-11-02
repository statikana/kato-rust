use std::fmt;

use crate::nodes::NodeBehavior;

pub struct ASTBoolean(pub bool);

impl NodeBehavior for ASTBoolean {
    fn name(&self) -> &'static str {
        "Boolean"
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
