pub trait Backend {

    // Initialize
    unsafe fn begin_draw(&mut self);
    unsafe fn flush(&mut self);

    unsafe fn get_display_size(&mut self) -> (u32,u32);

    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

    // Primitives
    unsafe fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, r: f32, g: f32, b: f32, a: f32);
}
