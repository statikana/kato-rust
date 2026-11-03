use typed::types::UnitType;

use crate::type_layouts::{Layout, HasLayout};


impl HasLayout for UnitType {
    fn layout(&self, _target: &super::Target) -> Option<Layout> {
        None
    }
}