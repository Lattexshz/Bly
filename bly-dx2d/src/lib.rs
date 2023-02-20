use bly_ac::Backend;
use windows::{
    core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*, Win32::UI::WindowsAndMessaging::*,
};

pub fn create_backend(hwnd: isize) -> std::result::Result<Direct2DBackend, ()> {
    let mut backend = Direct2DBackend::new(HWND(hwnd)).unwrap();
    match backend.render() {
        Ok(_) => {}
        Err(_) => {
            return Err(());
        }
    }
    Ok(backend)
}

pub struct Direct2DBackend {
    handle: HWND,
    factory: ID2D1Factory1,
    style: ID2D1StrokeStyle,
    target: Option<ID2D1HwndRenderTarget>,
    brush1: Option<ID2D1SolidColorBrush>,
    brush2: Option<ID2D1SolidColorBrush>,
}

impl Backend for Direct2DBackend {
    unsafe fn begin_draw(&mut self) {
        let target = self.target.as_ref().unwrap();
        target.BeginDraw();
    }

    unsafe fn flush(&mut self) {
        let target = self.target.as_ref().unwrap();
        target
            .EndDraw(std::ptr::null_mut(), std::ptr::null_mut())
            .unwrap();
    }

    unsafe fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let target = self.target.as_ref().unwrap();
        target.Clear(&D2D1_COLOR_F { r, g, b, a });
    }

    unsafe fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, r: f32, g: f32, b: f32, a: f32) {
        let target = self.target.as_ref().unwrap();
        let color = D2D1_COLOR_F {
            r,
            g,
            b,
            a,
        };

        let properties = D2D1_BRUSH_PROPERTIES {
            opacity: 0.8,
            transform: Matrix3x2::identity(),
        };


        let render_size = target.GetSize();
        let brush1 = &target.CreateSolidColorBrush(&color, &properties).unwrap();
        // Draw two rectangles.
        let rx = render_size.width / 2.0;
        let ry = render_size.height / 2.0;

        let rect1 = D2D_RECT_F {
            left:width,
            right:x,
            top:y,
            bottom:height,
        };

        target.FillRectangle(&rect1, brush1);
    }
}

impl Direct2DBackend {
    fn new(hwnd: HWND) -> Result<Self> {
        let factory = create_factory()?;
        let style = create_style(&factory)?;
        Ok(Self {
            handle: hwnd,
            factory,
            style,
            target: None,
            brush1: None,
            brush2: None,
        })
    }

    pub fn render(&mut self) -> Result<()> {
        if self.target.is_none() {
            let hwnd = self.handle;
            let mut rect = RECT::default();

            unsafe {
                GetClientRect(self.handle, &mut rect);
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

            let gray = D2D1_COLOR_F {
                r: 0.345,
                g: 0.423,
                b: 0.463,
                a: 1.0,
            };
            let red = D2D1_COLOR_F {
                r: 0.941,
                g: 0.353,
                b: 0.392,
                a: 1.0,
            };

            let properties = D2D1_BRUSH_PROPERTIES {
                opacity: 0.8,
                transform: Matrix3x2::identity(),
            };

            let target = unsafe {
                self.factory
                    .CreateHwndRenderTarget(&render_properties, &hwnd_render_properties)?
            };
            let brush1 = unsafe { target.CreateSolidColorBrush(&gray, &properties)? };
            let brush2 = unsafe { target.CreateSolidColorBrush(&red, &properties)? };

            self.target = Some(target);
            self.brush1 = Some(brush1);
            self.brush2 = Some(brush2);
        }

        let target = self.target.as_ref().unwrap();
        unsafe {
            target.BeginDraw();
            target.EndDraw(std::ptr::null_mut(), std::ptr::null_mut())?;
        };

        Ok(())
    }

    pub unsafe fn destroy(&mut self) {
        self.render().unwrap();
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

fn create_style(factory: &ID2D1Factory1) -> Result<ID2D1StrokeStyle> {
    let props = D2D1_STROKE_STYLE_PROPERTIES {
        startCap: D2D1_CAP_STYLE_ROUND,
        endCap: D2D1_CAP_STYLE_TRIANGLE,
        ..Default::default()
    };

    unsafe { factory.CreateStrokeStyle(&props, &[]) }
}
