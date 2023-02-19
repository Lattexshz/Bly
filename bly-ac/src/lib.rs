pub trait Backend {
    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);
}
