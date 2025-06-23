use core::convert::TryInto;
use core::fmt;
use embedded_graphics::pixelcolor::{Rgb565, Rgb888};
use lvgl_sys::lv_coord_t;

pub type LvResult<T> = Result<T, LvError>;

pub const LV_SIZE_CONTENT: u32 = 2001 | lvgl_sys::LV_COORD_TYPE_SPEC;

pub fn lv_pct(pct: lv_coord_t) -> lv_coord_t {
    unsafe{
        lvgl_sys::lv_pct(pct)
    }
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LvError::InvalidReference => "Accessed invalid reference or ptr",
                LvError::Uninitialized => "LVGL uninitialized",
                LvError::LvOOMemory => "LVGL out of memory",
                LvError::AlreadyInUse => "Resource already in use",
            }
        )
    }
}

/*
impl From<DisplayError> for LvError {
    fn from(err: DisplayError) -> Self {
        use LvError::*;
        match err {
            DisplayError::NotAvailable => Uninitialized,
            DisplayError::FailedToRegister => InvalidReference,
            DisplayError::NotRegistered => Uninitialized,
        }
    }
}*/

/*
impl From<LvError> for DisplayError {
    fn from(err: LvError) -> Self {
        use DisplayError::*;
        match err {
            LvError::InvalidReference => FailedToRegister,
            LvError::Uninitialized => NotAvailable,
            LvError::LvOOMemory => FailedToRegister,
            LvError::AlreadyInUse => FailedToRegister,
        }
    }
}*/

/// An LVGL color. Equivalent to `lv_color_t`.
#[derive(Copy, Clone)]
pub struct Color {
    pub(crate) raw: lvgl_sys::lv_color_t,
}

impl Default for Color {
    fn default() -> Self {
        let raw = unsafe { lvgl_sys::lv_color_black() };
        Self { raw }
    }
}

impl Color {
    /// Creates a `Color` from red, green, and blue values.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let raw = unsafe { lvgl_sys::lv_color_make(r, g, b) };
        Self { raw }
    }
    /// Creates a `Color` from a native `lv_color_t` instance.
    pub fn from_raw(raw: lvgl_sys::lv_color_t) -> Self {
        Self { raw }
    }
    /// Returns the value of the red channel.
    pub fn r(&self) -> u8 {
        (self.raw.red) as u8
    }
    /// Returns the value of the green channel.
    pub fn g(&self) -> u8 {
        self.raw.green as u8
    }
    /// Returns the value of the blue channel.
    pub fn b(&self) -> u8 {
        self.raw.blue as u8
    }
}

impl From<Color> for Rgb888 {
    fn from(color: Color) -> Self {
        Rgb888::new(
            color.raw.red as u8,
            color.raw.green as u8,
            color.raw.blue as u8,
        )
    }
}

impl From<Color> for Rgb565 {
    fn from(color: Color) -> Self {
        Rgb565::new(
            color.raw.red as u8,
            color.raw.green as u8,
            color.raw.blue as u8,
        )
    }
}

impl From<Color> for lvgl_sys::lv_color_t {
    fn from(val: Color) -> Self {
        val.raw
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

impl From<Align> for lvgl_sys::lv_align_t {
    fn from(value: Align) -> lvgl_sys::lv_align_t {
        let native = match value {
            Align::Center => lvgl_sys::lv_align_t_LV_ALIGN_CENTER,
            Align::TopLeft => lvgl_sys::lv_align_t_LV_ALIGN_TOP_LEFT,
            Align::TopMid => lvgl_sys::lv_align_t_LV_ALIGN_TOP_MID,
            Align::TopRight => lvgl_sys::lv_align_t_LV_ALIGN_TOP_RIGHT,
            Align::BottomLeft => lvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_LEFT,
            Align::BottomMid => lvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_MID,
            Align::BottomRight => lvgl_sys::lv_align_t_LV_ALIGN_BOTTOM_RIGHT,
            Align::LeftMid => lvgl_sys::lv_align_t_LV_ALIGN_LEFT_MID,
            Align::RightMid => lvgl_sys::lv_align_t_LV_ALIGN_RIGHT_MID,
            Align::OutTopLeft => lvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_LEFT,
            Align::OutTopMid => lvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_MID,
            Align::OutTopRight => lvgl_sys::lv_align_t_LV_ALIGN_OUT_TOP_RIGHT,
            Align::OutBottomLeft => lvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_LEFT,
            Align::OutBottomMid => lvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_MID,
            Align::OutBottomRight => lvgl_sys::lv_align_t_LV_ALIGN_OUT_BOTTOM_RIGHT,
            Align::OutLeftTop => lvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_TOP,
            Align::OutLeftMid => lvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_MID,
            Align::OutLeftBottom => lvgl_sys::lv_align_t_LV_ALIGN_OUT_LEFT_BOTTOM,
            Align::OutRightTop => lvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_TOP,
            Align::OutRightMid => lvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_MID,
            Align::OutRightBottom => lvgl_sys::lv_align_t_LV_ALIGN_OUT_RIGHT_BOTTOM,
        };
        native as lvgl_sys::lv_align_t
    }
}

pub enum TextAlign {
    Auto,
    Center,
    Left,
    Right,
}

impl From<TextAlign> for lvgl_sys::lv_align_t {
    fn from(value: TextAlign) -> Self {
        let native = match value {
            TextAlign::Auto => lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_AUTO,
            TextAlign::Center => lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_CENTER,
            TextAlign::Left => lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_LEFT,
            TextAlign::Right => lvgl_sys::lv_text_align_t_LV_TEXT_ALIGN_RIGHT,
        };
        native as lvgl_sys::lv_align_t
    }
}

/// Boolean for determining whether animations are enabled.
pub enum AnimationState {
    ON,
    OFF,
}

impl From<AnimationState> for lvgl_sys::lv_anim_enable_t {
    fn from(anim: AnimationState) -> Self {
        match anim {
            AnimationState::ON => lvgl_sys::lv_anim_enable_t_LV_ANIM_ON,
            AnimationState::OFF => lvgl_sys::lv_anim_enable_t_LV_ANIM_OFF,
        }
    }
}

#[repr(u32)]
pub enum LabelLongMode {
    Clip = lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_CLIP,
    Dot = lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_DOT,
    Scroll = lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_SCROLL,
    ScrollCircular = lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_SCROLL_CIRCULAR,
    Wrap = lvgl_sys::lv_label_long_mode_t_LV_LABEL_LONG_WRAP,
}

impl From<LabelLongMode> for u8 {
    fn from(value: LabelLongMode) -> Self {
        unsafe { (value as u32).try_into().unwrap_unchecked() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_properties_accessible() {
        let color = Color::from_rgb(206, 51, 255);

        if lvgl_sys::LV_COLOR_DEPTH == 32 {
            assert_eq!(color.r(), 206);
            assert_eq!(color.g(), 51);
            assert_eq!(color.b(), 255);
        } else if lvgl_sys::LV_COLOR_DEPTH == 16 {
            assert_eq!(color.r(), 25);
            assert_eq!(color.g(), 12);
            assert_eq!(color.b(), 31);
        }
    }
}
