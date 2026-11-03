use cranelift_codegen::ir::types::I8;
use typed::types::BooleanType;
use super::{ Layout, HasLayout };

impl HasLayout for BooleanType {
    fn layout(&self, _target: &super::Target) -> Option<Layout> {
        Some(Layout {
            size: 1,
            align: 1,
            clif_type: I8
        })
    }
}