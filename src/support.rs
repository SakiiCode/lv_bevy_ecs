//! Utility structs and functions

use embedded_graphics::pixelcolor::{BinaryColor, Gray8, Rgb565, Rgb888};

pub const LV_SIZE_CONTENT: u32 = lightvgl_sys::LV_COORD_MAX | lightvgl_sys::LV_COORD_TYPE_SPEC;

#[macro_export]
macro_rules! cstr {
    ($txt:expr) => {{
        const STR: &[u8] = concat!($txt, "\0").as_bytes();
        unsafe { CStr::from_bytes_with_nul_unchecked(STR) }
    }};
}

pub fn lv_pct(pct: lightvgl_sys::lv_coord_t) -> lightvgl_sys::lv_coord_t {
    unsafe { lightvgl_sys::lv_pct(pct) }
}

pub fn lv_dpx(n: i32) -> i32 {
    unsafe { lightvgl_sys::lv_dpx(n) }
}

pub fn lv_color_make(r: u8, g: u8, b: u8) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_color_make(r, g, b) }
}

#[cfg(LV_USE_GRID)]
pub fn lv_grid_fr(x: u8)->i32 {
    unsafe { lightvgl_sys::lv_grid_fr(x) }
}

pub trait LvglColorFormat {
    fn as_lv_color_format_t() -> lightvgl_sys::lv_color_format_t;
}

impl LvglColorFormat for Rgb565 {
    fn as_lv_color_format_t() -> lightvgl_sys::lv_color_format_t {
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565
    }
}

impl LvglColorFormat for Rgb888 {
    fn as_lv_color_format_t() -> lightvgl_sys::lv_color_format_t {
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB888
    }
}

impl LvglColorFormat for Gray8 {
    fn as_lv_color_format_t() -> lightvgl_sys::lv_color_format_t {
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_L8
    }
}

impl LvglColorFormat for BinaryColor {
    fn as_lv_color_format_t() -> lightvgl_sys::lv_color_format_t {
        crate::warn!("Monochrome buffers are not supported. Proceed with caution!");
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I1
    }
}

/// Possible LVGL alignments for widgets.
pub enum Align {
    Center,
    TopLeft,
    TopMid,
    TopRight,
    BottomLeft,
    BottomMid,
    BottomRight,
    LeftMid,
    RightMid,
    OutTopLeft,
    OutTopMid,
    OutTopRight,
    OutBottomLeft,
    OutBottomMid,
    OutBottomRight,
    OutLeftTop,
    OutLeftMid,
    OutLeftBottom,
    OutRightTop,
    OutRightMid,
    OutRightBottom,
}

impl From<Align> for lightvgl_sys::lv_align_t {
    fn from(value: Align) -> lightvgl_sys::lv_align_t {
        let native = match value {
            Align::Center => lightvgl_sys::lv_align_t_LV_ALIGN_CENTER,
            Align::TopLeft => lightvgl_sys::lv_align_t_LV_ALIGN_TOP_LEFT,
            Align::TopMid => lightvgl_sys::lv_align_t_LV_ALIGN_TOP_MID,
            Align::TopRight => lightvgl_sys::lv_align_t_LV_ALIGN_TOP_RIGHT,
            Align::BottomLeft => lightvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_LEFT,
            Align::BottomMid => lightvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_MID,
            Align::BottomRight => lightvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_RIGHT,
            Align::LeftMid => lightvgl_sys::lv_align_t_LV_ALIGN_LEFT_MID,
            Align::RightMid => lightvgl_sys::lv_align_t_LV_ALIGN_RIGHT_MID,
            Align::OutTopLeft => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_LEFT,
            Align::OutTopMid => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_MID,
            Align::OutTopRight => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_RIGHT,
            Align::OutBottomLeft => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_LEFT,
            Align::OutBottomMid => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_MID,
            Align::OutBottomRight => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_RIGHT,
            Align::OutLeftTop => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_TOP,
            Align::OutLeftMid => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_MID,
            Align::OutLeftBottom => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_BOTTOM,
            Align::OutRightTop => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_TOP,
            Align::OutRightMid => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_MID,
            Align::OutRightBottom => lightvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_BOTTOM,
        };
        native as lightvgl_sys::lv_align_t
    }
}

pub enum TextAlign {
    Auto,
    Center,
    Left,
    Right,
}

impl From<TextAlign> for lightvgl_sys::lv_align_t {
    fn from(value: TextAlign) -> Self {
        let native = match value {
            TextAlign::Auto => lightvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_AUTO,
            TextAlign::Center => lightvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_CENTER,
            TextAlign::Left => lightvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_LEFT,
            TextAlign::Right => lightvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_RIGHT,
        };
        native as lightvgl_sys::lv_align_t
    }
}

#[repr(u32)]
pub enum LabelLongMode {
    Clip = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_CLIP,
    Dots = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_DOTS,
    Scroll = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_SCROLL,
    ScrollCircular = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_SCROLL_CIRCULAR,
    Wrap = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_WRAP,
}

impl From<LabelLongMode> for lightvgl_sys::lv_label_long_mode_t {
    fn from(value: LabelLongMode) -> Self {
        value as lightvgl_sys::lv_label_long_mode_t
    }
}

#[repr(u32)]
pub enum OpacityLevel {
    Transparent = lightvgl_sys::_lv_opacity_level_t_LV_OPA_TRANSP,
    Percent10 = lightvgl_sys::_lv_opacity_level_t_LV_OPA_10,
    Percent20 = lightvgl_sys::_lv_opacity_level_t_LV_OPA_20,
    Percent30 = lightvgl_sys::_lv_opacity_level_t_LV_OPA_30,
    Percent40 = lightvgl_sys::_lv_opacity_level_t_LV_OPA_40,
    Percent50 = lightvgl_sys::_lv_opacity_level_t_LV_OPA_50,
    Percent60 = lightvgl_sys::_lv_opacity_level_t_LV_OPA_60,
    Percent70 = lightvgl_sys::_lv_opacity_level_t_LV_OPA_70,
    Percent80 = lightvgl_sys::_lv_opacity_level_t_LV_OPA_80,
    Percent90 = lightvgl_sys::_lv_opacity_level_t_LV_OPA_90,
    Cover = lightvgl_sys::_lv_opacity_level_t_LV_OPA_COVER,
}

impl From<OpacityLevel> for lightvgl_sys::_lv_opacity_level_t {
    fn from(value: OpacityLevel) -> Self {
        value as lightvgl_sys::_lv_opacity_level_t
    }
}
