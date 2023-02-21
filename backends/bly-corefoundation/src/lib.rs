use bly_ac::Backend;

pub fn create_backend() -> CoreFoundationBackend {
    CoreFoundationBackend {
        
    }
}

pub struct CoreFoundationBackend {
    
}

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

    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        
    }

    unsafe fn draw_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32, r: f32, g: f32, b: f32, a: f32) {

    }

    unsafe fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, stroke: f32, r: f32, g: f32, b: f32, a: f32) {
        todo!()
    }
}