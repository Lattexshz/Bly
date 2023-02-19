use bly_ac::Backend;

pub fn create_backend() -> CoreFoundationBackend {
    CoreFoundationBackend {
        
    }
}

pub struct CoreFoundationBackend {
    
}

impl Backend for CoreFoundationBackend {
    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        
    }

    unsafe fn draw_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32, r: f32, g: f32, b: f32, a: f32) {

    }
}