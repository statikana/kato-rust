use cranelift_codegen::ir::types::F64;
use typed::types::FloatType;

use super::{HasLayout, Layout, Target};

impl HasLayout for FloatType {
    fn layout(&self, _target: &Target) -> Option<Layout> {
        Some(Layout {
            size: 64,
            align: 64,
            clif_type: F64,
        })
    }
}
