use bly_ac::Backend;

pub fn create_backend() -> CoreFoundationBackend {
    CoreFoundationBackend {}
}

pub struct CoreFoundationBackend {}

impl Backend for CoreFoundationBackend {
    unsafe fn begin_draw(&mut self) {
        todo!()
    }

    unsafe fn flush(&mut self) {
        todo!()
    }

    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        todo!()
    }

    unsafe fn clear(&mut self, _r: f32, _g: f32, _b: f32, _a: f32) {}

    unsafe fn draw_ellipse(
        &mut self,
        _x: f32,
        _y: f32,
        _radius_x: f32,
        _radius_y: f32,
        _r: f32,
        _g: f32,
        _b: f32,
        _a: f32,
    ) {
        todo!()
    }

    unsafe fn draw_rect(
        &mut self,
        _left: f32,
        _top: f32,
        _right: f32,
        _bottom: f32,
        _r: f32,
        _g: f32,
        _b: f32,
        _a: f32,
    ) {
    }

    unsafe fn draw_line(
        &mut self,
        _x1: f32,
        _y1: f32,
        _x2: f32,
        _y2: f32,
        _stroke: f32,
        _r: f32,
        _g: f32,
        _b: f32,
        _a: f32,
    ) {
        todo!()
    }
}
