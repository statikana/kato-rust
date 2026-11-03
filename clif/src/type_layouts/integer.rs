use cranelift_codegen::ir::types::I64;
use typed::types::IntegerType;

use super::{HasLayout, Layout, Target};

impl HasLayout for IntegerType {
    fn layout(&self, _target: &Target) -> Option<Layout> {
        Some(Layout { size: 64, align: 64, clif_type: I64 })
    }
}
