use cranelift_codegen::ir::types::{ I8 };
use typed::types::PtrType;
use super::{HasLayout, Layout, Target};

impl HasLayout for PtrType {
    fn layout(&self, target: &Target) -> Option<Layout> {
        Some(Layout { 
            size: target.pointer_size,
            align: target.pointer_size,
            clif_type: I8 // or I4, but doesn't exist
         })
    }
}
