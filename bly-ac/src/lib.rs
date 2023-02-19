pub trait Backend {
    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);

    // Primitives
    unsafe fn draw_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32, r: f32, g: f32, b: f32, a: f32);
}
