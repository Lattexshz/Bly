use std::ffi::c_ulong;
use x11::xlib::{Display, XGetGeometry};

#[inline]
pub(crate) unsafe fn get_xlib_window_size(
    display: *mut Display,
    window: c_ulong,
) -> (c_ulong, c_ulong) {
    let mut width = 0;
    let mut height = 0;
    let mut dummy = 0;
    let mut c_int_dummy = 0;
    let mut c_uint_dummy = 0;
    XGetGeometry(
        display,
        window,
        &mut dummy,
        &mut c_int_dummy,
        &mut c_int_dummy,
        &mut width,
        &mut height,
        &mut c_uint_dummy,
        &mut c_uint_dummy,
    );

    (width.into(), height.into())
}
