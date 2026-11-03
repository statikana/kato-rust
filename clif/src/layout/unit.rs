use typed::types::UnitType;

use crate::layout::{Layout, HasLayout};


impl HasLayout for UnitType {
    fn layout(&self, _target: &super::Target) -> Option<Layout> {
        None
    }
}