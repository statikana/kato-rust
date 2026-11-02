use std::fmt;

use crate::nodes::NodeBehavior;

pub struct ASTUnit;

impl NodeBehavior for ASTUnit {
    fn name(&self) -> &'static str {
        "Unit"
    }

    fn fmt_labeled(
        &self,
        label: &str,
        _children: &[&dyn fmt::Debug],
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(label)
    }
}
