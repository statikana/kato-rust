use typed::types::UnitType;

use crate::type_layouts::{HasLayout, Layout};

impl HasLayout for UnitType {
    fn layout(&self, _target: &super::Target) -> Option<Layout> {
        None
    }
}
