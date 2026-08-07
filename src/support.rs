//! Utility structs and functions

use core::{
    ffi::c_void,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use embedded_graphics::pixelcolor::{
    Gray8, IntoStorage, Rgb565, Rgb888, RgbColor,
    raw::{RawU8, RawU16},
};

pub const LV_SIZE_CONTENT: u32 = lightvgl_sys::LV_COORD_MAX | lightvgl_sys::LV_COORD_TYPE_SPEC;

#[macro_export]
macro_rules! cstr {
    ($txt:expr) => {{
        const STR: &[u8] = concat!($txt, "\0").as_bytes();
        unsafe { CStr::from_bytes_with_nul_unchecked(STR) }
    }};
}

#[inline]
pub(crate) fn assert_lv_is_initialized() {
    unsafe {
        assert!(
            lightvgl_sys::lv_is_initialized(),
            "LVGL was not initialized! Add lv_init(); to the beginning of main()"
        );
    }
}

pub trait LvglColorFormat {
    fn as_lv_color_format_t() -> lightvgl_sys::lv_color_format_t;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct LvGray8(u8);

impl LvglColorFormat for LvGray8 {
    #[inline]
    fn as_lv_color_format_t() -> lightvgl_sys::lv_color_format_t {
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_L8
    }
}

impl From<Gray8> for LvGray8 {
    fn from(value: Gray8) -> Self {
        Self(value.into_storage())
    }
}

impl From<LvGray8> for Gray8 {
    fn from(value: LvGray8) -> Self {
        RawU8::new(value.0).into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct LvRgb565(u16);

impl LvglColorFormat for LvRgb565 {
    #[inline]
    fn as_lv_color_format_t() -> lightvgl_sys::lv_color_format_t {
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565
    }
}

impl From<Rgb565> for LvRgb565 {
    fn from(value: Rgb565) -> Self {
        Self(value.into_storage())
    }
}

impl From<LvRgb565> for Rgb565 {
    fn from(value: LvRgb565) -> Self {
        RawU16::new(value.0).into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct U24(u8, u8, u8);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct LvRgb888(U24);

impl LvglColorFormat for LvRgb888 {
    #[inline]
    fn as_lv_color_format_t() -> lightvgl_sys::lv_color_format_t {
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB888
    }
}

impl From<Rgb888> for LvRgb888 {
    fn from(value: Rgb888) -> Self {
        Self(U24(value.r(), value.g(), value.b()))
    }
}

impl From<LvRgb888> for Rgb888 {
    fn from(value: LvRgb888) -> Self {
        Rgb888::new(value.0.2, value.0.1, value.0.0)
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
    #[inline]
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

#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
pub enum LabelLongMode {
    Clip = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_CLIP,
    Dots = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_DOTS,
    Scroll = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_SCROLL,
    ScrollCircular = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_SCROLL_CIRCULAR,
    Wrap = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_MODE_WRAP,
}

impl From<LabelLongMode> for lightvgl_sys::lv_label_long_mode_t {
    #[inline]
    fn from(value: LabelLongMode) -> Self {
        value as lightvgl_sys::lv_label_long_mode_t
    }
}

#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
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
    #[inline]
    fn from(value: OpacityLevel) -> Self {
        value as lightvgl_sys::_lv_opacity_level_t
    }
}

pub trait Mutability {}

pub struct Mut;

impl Mutability for Mut {}

pub struct Const;

impl Mutability for Const {}

pub struct Void<T: Mutability> {
    addr: usize,
    phantom: PhantomData<T>,
}

impl Void<Const> {
    pub fn as_c_void(self) -> *const c_void {
        core::ptr::with_exposed_provenance(self.addr)
    }
}

impl Void<Mut> {
    pub fn as_c_void(self) -> *mut c_void {
        core::ptr::with_exposed_provenance_mut(self.addr)
    }
}

pub trait ToVoid {
    fn to_void(&self) -> Void<Const>;
}

pub trait ToVoidMut {
    fn to_void_mut(&mut self) -> Void<Mut>;
}

impl<T: ?Sized + Deref> ToVoid for T {
    fn to_void(&self) -> Void<Const> {
        Void {
            addr: (&raw const **self).expose_provenance(),
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized + DerefMut> ToVoidMut for T {
    fn to_void_mut(&mut self) -> Void<Mut> {
        Void {
            addr: (&raw mut **self).expose_provenance(),
            phantom: PhantomData,
        }
    }
}
