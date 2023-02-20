//! Direct2D backend for Bly

use bly_ac::Backend;
use windows::{
    core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*, Win32::UI::WindowsAndMessaging::*,
};

/// Create the backend from hwnd. This is the only method available to the public.
pub fn create_backend(hwnd: isize) -> std::result::Result<Direct2DBackend, ()> {
    let backend = Direct2DBackend::new(HWND(hwnd)).unwrap();

    Ok(backend)
}

pub struct Direct2DBackend {
    handle: HWND,

    width: u32,
    height: u32,

    factory: ID2D1Factory1,
    target: ID2D1HwndRenderTarget,
}

impl Backend for Direct2DBackend {
    unsafe fn begin_draw(&mut self) {
        self.update_target();
        self.target.BeginDraw();
    }

    unsafe fn flush(&mut self) {
        self.target
            .EndDraw(std::ptr::null_mut(), std::ptr::null_mut())
            .unwrap();
    }

    unsafe fn get_display_size(&mut self) -> (u32, u32) {
        unsafe {
            let size = self.target.GetSize();
            (size.width as u32,size.height as u32)
        }
    }

    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.target.Clear(&D2D1_COLOR_F { r, g, b, a });
    }

    unsafe fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, r: f32, g: f32, b: f32, a: f32) {
        let color = D2D1_COLOR_F {
            r,
            g,
            b,
            a,
        };

        let properties = D2D1_BRUSH_PROPERTIES {
            opacity: a,
            transform: Matrix3x2::identity(),
        };

        let brush1 = &self.target.CreateSolidColorBrush(&color, &properties).unwrap();

        let rect1 = D2D_RECT_F {
            left:x,
            right:x+width,
            top:y,
            bottom:y+height
        };

        self.target.FillRectangle(&rect1, brush1);
    }
}

fn create_target(hwnd:HWND,factory:&ID2D1Factory1) -> (ID2D1HwndRenderTarget,u32,u32) {
    let mut rect = RECT::default();

    unsafe {
        GetClientRect(hwnd, &mut rect);
    }

    let d2d_rect = D2D_SIZE_U {
        width: (rect.right - rect.left) as u32,
        height: (rect.bottom - rect.top) as u32,
    };

    let render_properties = D2D1_RENDER_TARGET_PROPERTIES::default();

    let hwnd_render_properties = D2D1_HWND_RENDER_TARGET_PROPERTIES {
        hwnd,
        pixelSize: d2d_rect,
        presentOptions: D2D1_PRESENT_OPTIONS_NONE,
    };

    let target = unsafe {
        factory
            .CreateHwndRenderTarget(&render_properties, &hwnd_render_properties).unwrap()
    };
    (target,(rect.right - rect.left) as u32,(rect.bottom - rect.top) as u32)
}


impl Direct2DBackend {
    /// Create a new backend
    fn new(hwnd: HWND) -> Result<Self> {
        let factory = create_factory()?;
        let (target,width,height) = create_target(hwnd,&factory);
        Ok(Self {
            handle: hwnd,
            width,
            height,
            factory,
            target,
        })
    }

    /// Regenerate Target (to accommodate window resizing)
    fn update_target(&mut self){
        let mut rect = RECT::default();

        unsafe {
            GetClientRect(self.handle, &mut rect);
        }

        let d2d_rect = D2D_SIZE_U {
            width: (rect.right - rect.left) as u32,
            height: (rect.bottom - rect.top) as u32,
        };

        // Re-create Target only when the window size changes
        if self.width != d2d_rect.width || self.height != d2d_rect.height {
            self.width = d2d_rect.width;
            self.height = d2d_rect.height;

            let render_properties = D2D1_RENDER_TARGET_PROPERTIES::default();

            let hwnd_render_properties = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd:self.handle,
                pixelSize: d2d_rect,
                presentOptions: D2D1_PRESENT_OPTIONS_NONE,
            };

            self.target = unsafe {
                self.factory
                    .CreateHwndRenderTarget(&render_properties, &hwnd_render_properties).unwrap()
            };
        }
    }
}

fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    let mut result = None;

    unsafe {
        D2D1CreateFactory(
            D2D1_FACTORY_TYPE_SINGLE_THREADED,
            &ID2D1Factory1::IID,
            &options,
            std::mem::transmute(&mut result),
        )
        .map(|()| result.unwrap())
    }
}