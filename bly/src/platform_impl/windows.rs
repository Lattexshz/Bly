use crate::Color;
use std::ffi::{c_int, c_void};
use winapi::shared::minwindef::BYTE;
use winapi::shared::windef::{COLORREF, HWND};
use winapi::um::wingdi::{
    GetStockObject, Rectangle, SelectObject, SetDCBrushColor, SetDCPenColor, DC_BRUSH, DC_PEN, RGB,
};
use winapi::um::winuser::{GetDC, GetSystemMetrics, ReleaseDC, SendMessageA, SM_CXSCREEN, SM_CYSCREEN, WM_ERASEBKGND};
use crate::primitive::Rectangle;

pub fn get_color(color: Color) -> COLORREF {
    match color {
        Color::White => RGB(255, 255, 255),
        Color::WhiteGray => RGB(240, 240, 240),
        Color::Gray => RGB(128, 128, 128),
        Color::Black => RGB(0, 0, 0),
        Color::Red => RGB(255, 0, 0),
        Color::Green => RGB(0, 255, 0),
        Color::Blue => RGB(0, 0, 255),
        Color::Rgba(r, g, b, a) => RGB(r as BYTE, g as BYTE, b as BYTE),
    }
}

pub unsafe fn _fill(hwnd: *mut c_void, color: COLORREF) {
    let hdc = GetDC(hwnd as HWND);
    // Set brush color...
    SetDCBrushColor(hdc, color);
    SetDCPenColor(hdc, color);

    SelectObject(hdc, GetStockObject(DC_PEN as c_int));
    SelectObject(hdc, GetStockObject(DC_BRUSH as c_int));

    Rectangle(
        hdc,
        0,
        0,
        GetSystemMetrics(SM_CXSCREEN),
        GetSystemMetrics(SM_CYSCREEN),
    );

    // Release hdc
    ReleaseDC(hwnd as HWND, hdc);
}

pub unsafe fn _draw_rect(hwnd: *mut c_void, rect:Rectangle) {
    let hdc = GetDC(hwnd as HWND);
    let color = get_color(rect.color);
    // Set brush color...
    SetDCBrushColor(hdc, color);
    SetDCPenColor(hdc, color);

    SelectObject(hdc, GetStockObject(DC_PEN as c_int));
    SelectObject(hdc, GetStockObject(DC_BRUSH as c_int));

    Rectangle(
        hdc,
        rect.vertex.0 as c_int,
        rect.vertex.1 as c_int,
        rect.vertex.2 as c_int,
        rect.vertex.3 as c_int
    );

    // Release hdc
    ReleaseDC(hwnd as HWND, hdc);
}
