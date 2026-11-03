use super::{HasLayout, Layout};
use cranelift_codegen::ir::types::I8;
use typed::types::BooleanType;

impl HasLayout for BooleanType {
    fn layout(&self, _target: &super::Target) -> Option<Layout> {
        Some(Layout {
            size: 1,
            align: 1,
            clif_type: I8,
        })
    }
}
