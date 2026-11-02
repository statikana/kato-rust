use std::fmt;

use crate::nodes::NodeBehavior;

pub struct ASTFloat(pub f64);

impl NodeBehavior for ASTFloat {
    fn name(&self) -> &'static str {
        "Float"
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
