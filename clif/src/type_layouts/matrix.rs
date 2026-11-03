use typed::types::MatrixType;

use super::{HasLayout, Layout, Target};

impl HasLayout for MatrixType {
    fn layout(&self, target: &Target) -> Option<Layout> {
        let element_layout = self.dtype.layout(target)?;

        let n_lanes = vector_lanes(target, &element_layout)?;

        // just the number of elements in the matrix
        let n_elements_jagged = self.shape.iter().fold(1, |cur, dim_size| cur * dim_size);

        // padded number of elements such that it fits into the number
        // of lanes for this matrix
        let n_elements_padded = n_elements_jagged.next_multiple_of(n_lanes);

        // e.g. I8.by(32) => I8X32
        let clif_type = element_layout.clif_type.by(n_lanes.try_into().ok()?)?;
        let size = n_elements_padded * element_layout.size;
        let align = element_layout.align;

        Some(Layout {
            size,
            align,
            clif_type,
        })
    }
}

fn vector_lanes(target: &Target, element_layout: &Layout) -> Option<usize> {
    let elem_size = element_layout.clif_type.bits();
    target
        .register_width
        .checked_div(elem_size.try_into().ok()?)
        .filter(|&n| n > 1)
}
