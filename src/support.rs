//! Utility structs and functions

use core::convert::TryInto;
use core::fmt;
use cstr_core::{CString, cstr};
use embedded_graphics::pixelcolor::{BinaryColor, Gray8, Rgb565, Rgb888};
use lightvgl_sys::{lv_coord_t, lv_log_level_t};
use std::error::Error;

use crate::functions::lv_log_add;

pub type LvResult<T> = Result<T, LvError>;

pub const LV_SIZE_CONTENT: u32 = 2001 | lightvgl_sys::LV_COORD_TYPE_SPEC;

#[macro_export]
macro_rules! func {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name[..name.len() - 3].split("::").last().unwrap()
    }};
}

#[macro_export]
macro_rules! lv_log_trace {
    ($msg:literal) => {
        lv_log_add(
            LogLevel::Trace,
            cstr!(file!()),
            line!(),
            CString::new(func!()).unwrap().as_c_str(),
            cstr!($msg),
        );
    };
}

#[macro_export]
macro_rules! lv_log_info {
    ($msg:literal) => {
        lv_log_add(
            LogLevel::Info,
            cstr!(file!()),
            line!(),
            CString::new(func!()).unwrap().as_c_str(),
            cstr!($msg),
        );
    };
}

#[macro_export]
macro_rules! lv_log_warn {
    ($msg:literal) => {
        lv_log_add(
            LogLevel::Warn,
            cstr!(file!()),
            line!(),
            CString::new(func!()).unwrap().as_c_str(),
            cstr!($msg),
        );
    };
}

#[macro_export]
macro_rules! lv_log_error {
    ($msg:literal) => {
        lv_log_add(
            LogLevel::Error,
            cstr!(file!()),
            line!(),
            CString::new(func!()).unwrap().as_c_str(),
            cstr!($msg),
        );
    };
}

#[macro_export]
macro_rules! lv_log_user {
    ($msg:literal) => {
        lv_log_add(
            LogLevel::User,
            cstr!(file!()),
            line!(),
            CStr::from_bytes_with_nul(func!().as_bytes()).unwrap(),
            cstr!($msg),
        );
    };
}

pub fn lv_pct(pct: lv_coord_t) -> lv_coord_t {
    unsafe { lightvgl_sys::lv_pct(pct) }
}

/// Generic LVGL error. All other errors can be coerced into it.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LvError {
    InvalidReference,
    Uninitialized,
    LvOOMemory,
    AlreadyInUse,
}

impl fmt::Display for LvError {
    #[allow(deprecated)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for LvError {
    fn cause(&self) -> Option<&dyn Error> {
        Some(self)
    }
    fn description(&self) -> &str {
        match self {
            LvError::InvalidReference => "Accessed invalid reference or ptr",
            LvError::Uninitialized => "LVGL uninitialized",
            LvError::LvOOMemory => "LVGL out of memory",
            LvError::AlreadyInUse => "Resource already in use",
        }
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LvError::InvalidReference => Some(self),
            LvError::Uninitialized => Some(self),
            LvError::LvOOMemory => Some(self),
            LvError::AlreadyInUse => Some(self),
        }
    }
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
        lv_log_warn!("Monochrome buffers are not supported. Proceed with caution!");
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I1
    }
}

pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error,
    User,
    None,
}

impl From<LogLevel> for lv_log_level_t {
    fn from(value: LogLevel) -> Self {
        (match value {
            LogLevel::Trace => lightvgl_sys::LV_LOG_LEVEL_TRACE,
            LogLevel::Info => lightvgl_sys::LV_LOG_LEVEL_INFO,
            LogLevel::Warn => lightvgl_sys::LV_LOG_LEVEL_WARN,
            LogLevel::Error => lightvgl_sys::LV_LOG_LEVEL_ERROR,
            LogLevel::User => lightvgl_sys::LV_LOG_LEVEL_USER,
            LogLevel::None => lightvgl_sys::LV_LOG_LEVEL_NONE,
        }) as lv_log_level_t
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

/// Boolean for determining whether animations are enabled.
pub enum AnimationState {
    ON,
    OFF,
}

impl From<AnimationState> for lightvgl_sys::lv_anim_enable_t {
    fn from(anim: AnimationState) -> Self {
        match anim {
            AnimationState::ON => lightvgl_sys::lv_anim_enable_t_LV_ANIM_ON,
            AnimationState::OFF => lightvgl_sys::lv_anim_enable_t_LV_ANIM_OFF,
        }
    }
}

#[repr(u32)]
pub enum LabelLongMode {
    Clip = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_CLIP,
    Dot = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_DOT,
    Scroll = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_SCROLL,
    ScrollCircular = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_SCROLL_CIRCULAR,
    Wrap = lightvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_WRAP,
}

impl From<LabelLongMode> for u8 {
    fn from(value: LabelLongMode) -> Self {
        unsafe { (value as u32).try_into().unwrap_unchecked() }
    }
}
