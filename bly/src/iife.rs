//! Immediately Invoke Funcation Expression

use crate::{Bly, Color, Vec4};

pub fn clear(bly: &mut Bly, color: Color) {
    unsafe {
        let vec: Vec4 = color.into();
        bly.bdc.begin_draw();
        bly.bdc.clear(color);
        bly.bdc.flush();
    }
}
