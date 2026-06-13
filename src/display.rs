//! # Display
//!
//! The `embedded-graphics` crate is used to drive the screens. Examples use `embedded-graphics-simulator` to try them on PC.
//!
//! ## Simulator
//! ```
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # use embedded_graphics::prelude::*;
//! # use embedded_graphics_simulator::*;
//! # use lv_bevy_ecs::display::{DrawBuffer, Display};
//! # use lv_bevy_ecs::support::LvglColorFormat;
//! # use lv_bevy_ecs::sys::*;
//! #
//! lv_bevy_ecs::functions::lv_init();
//! const HOR_RES: usize = 800;
//! const VER_RES: usize = 480;
//! const LINE_HEIGHT: usize = 10;
//!
//! let mut sim_display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(HOR_RES as u32, VER_RES as u32));
//! let mut display = Display::new(HOR_RES, VER_RES);
//!
//! let buffer = DrawBuffer::<{ (HOR_RES * LINE_HEIGHT) as usize }, Rgb565>::new(HOR_RES, LINE_HEIGHT);
//! display.register(buffer, move |refresh| {
//!     // alternative (slower): sim_display.draw_iter(refresh.as_pixels()).unwrap();
//!     sim_display
//!         .fill_contiguous(&refresh.rectangle, refresh.colors.iter().cloned())
//!         .unwrap();
//! });
//!
//! unsafe {
//!     let mut default_display = Display::get_default();
//!     assert_eq!(default_display.get_horizontal_resolution(), HOR_RES as i32);
//!     assert_eq!(default_display.get_vertical_resolution(), VER_RES as i32);
//!     assert_eq!(default_display.get_color_format(), Rgb565::as_lv_color_format_t());
//! }
//! ```
//!
//! ## ESP32
//!
//! For code example on how to create a display on ESP32 with SPI display and touchscreen,
//! check out [lvgl-bevy-demo](https://github.com/SakiiCode/lvgl-bevy-demo).
//!
//! ## ESP32-P4
//!
//! For code example on how to create a display on a DSI screen,
//! check out [lvgl-bevy-demo-dsi](https://github.com/SakiiCode/lvgl-bevy-demo-dsi).

use ::alloc::boxed::Box;
use ::core::{marker::PhantomData, ptr::NonNull};
use alloc::borrow::ToOwned;

use embedded_graphics::{
    Pixel,
    prelude::{PixelColor, Point, Size},
    primitives::Rectangle,
};
use lightvgl_sys::{
    lv_color_format_t, lv_display_flush_ready, lv_display_get_user_data, lv_display_t,
    lv_draw_buf_t,
};

use crate::support::LvglColorFormat;

pub struct Display {
    raw: NonNull<lv_display_t>,
}

#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
pub enum RenderMode {
    Partial = lightvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_PARTIAL,
    Direct = lightvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_DIRECT,
    Full = lightvgl_sys::lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_FULL,
}

impl From<RenderMode> for lightvgl_sys::lv_display_render_mode_t {
    #[inline]
    fn from(value: RenderMode) -> Self {
        value as Self
    }
}

#[cfg_attr(windows, repr(i32))]
#[cfg_attr(not(windows), repr(u32))]
pub enum DisplayRotation {
    Deg0 = lightvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_0,
    Deg90 = lightvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_90,
    Deg180 = lightvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_180,
    Deg270 = lightvgl_sys::lv_display_rotation_t_LV_DISPLAY_ROTATION_270,
}

impl From<DisplayRotation> for lightvgl_sys::lv_disp_rotation_t {
    #[inline]
    fn from(value: DisplayRotation) -> Self {
        value as Self
    }
}

impl Display {
    pub fn new(hor_res: usize, ver_res: usize) -> Self {
        crate::support::assert_lv_is_initialized();
        unsafe {
            let raw = NonNull::new(lightvgl_sys::lv_display_create(
                hor_res.try_into().unwrap(),
                ver_res.try_into().unwrap(),
            ))
            .unwrap();
            Self { raw }
        }
    }

    /// Assigns a callback to `lv_display_set_flush_cb`
    /// ## Arguments
    ///  - `buffer` - [DrawBuffer] object that matches the [Display] color format
    ///  - `callback` - Function or closure that pushes the pixels to the screen
    #[expect(clippy::needless_pass_by_value)]
    pub fn register<F, const N: usize, C: LvglColorFormat>(
        &mut self,
        buffer: DrawBuffer<N, C>,
        callback: F,
    ) where
        F: FnMut(&mut DisplayRefresh<N, C>) + 'static,
    {
        let cf = C::as_lv_color_format_t();
        verify_color_format(cf);
        unsafe {
            lightvgl_sys::lv_display_set_draw_buffers(
                self.raw_mut(),
                buffer.raw.as_ptr(),
                ::core::ptr::null_mut(),
            );
            lightvgl_sys::lv_display_set_flush_cb(
                self.raw.as_ptr(),
                Some(disp_flush_trampoline::<F, N, C>),
            );
            lightvgl_sys::lv_display_set_user_data(
                self.raw.as_ptr(),
                Box::into_raw(Box::new(callback)).cast(),
            );
        }
        crate::info!("Display Registered");
    }

    /// Assigns a callback to `lv_display_set_flush_cb`
    /// ## Arguments
    ///  * `buffer` - `[u8]` buffer that is exactly *N* bytes long
    ///  * `render_mode` - Specifies the `lv_display_render_mode_t`
    ///  * `callback` - Function or closure that pushes the pixels to the screen
    pub fn register_raw<F, const N: usize, C: LvglColorFormat>(
        &mut self,
        buffer: &'static mut [u8],
        render_mode: RenderMode,
        callback: F,
    ) where
        F: FnMut(&mut DisplayRefresh<N, C>) + 'static,
    {
        let cf = C::as_lv_color_format_t();
        verify_color_format(cf);
        assert_eq!(buffer.len(), N);
        unsafe {
            lightvgl_sys::lv_display_set_buffers(
                self.raw_mut(),
                buffer.as_mut_ptr().cast(),
                ::core::ptr::null_mut(),
                N.try_into().unwrap(),
                render_mode.into(),
            );
            lightvgl_sys::lv_display_set_flush_cb(
                self.raw.as_ptr(),
                Some(disp_flush_trampoline::<F, N, C>),
            );
            lightvgl_sys::lv_display_set_user_data(
                self.raw.as_ptr(),
                Box::into_raw(Box::new(callback)).cast(),
            );
        }
        crate::info!("Display Registered");
    }

    #[inline]
    pub fn set_flush_wait_cb(&mut self, callback: Option<unsafe extern "C" fn(*mut lv_display_t)>) {
        unsafe {
            lightvgl_sys::lv_display_set_flush_wait_cb(self.raw_mut(), callback);
        }
    }

    #[inline]
    pub fn get_default() -> Self {
        unsafe {
            Self {
                raw: NonNull::new(lightvgl_sys::lv_display_get_default()).unwrap(),
            }
        }
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: DisplayRotation) {
        unsafe {
            lightvgl_sys::lv_display_set_rotation(self.raw_mut(), rotation.into());
        }
    }

    #[inline]
    pub fn raw(&self) -> *const lv_display_t {
        self.raw.as_ptr().cast_const()
    }

    #[inline]
    pub fn raw_mut(&mut self) -> *mut lv_display_t {
        self.raw.as_ptr()
    }

    #[inline]
    pub fn from_ptr(ptr: *mut lv_display_t) -> Self {
        Self {
            raw: NonNull::new(ptr).unwrap(),
        }
    }

    /// Creates a new `Display` from the given `*mut lv_display_t`
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    #[inline]
    pub unsafe fn from_ptr_unchecked(ptr: *mut lv_display_t) -> Self {
        unsafe {
            Self {
                raw: NonNull::new_unchecked(ptr),
            }
        }
    }
}

fn verify_color_format(cf: lv_color_format_t) {
    // these are not checked by LVGL, just produce a black screen
    match cf {
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_L8 => {
            assert_eq!(
                lightvgl_sys::LV_COLOR_DEPTH,
                8,
                "LV_COLOR_DEPTH must be set to 8"
            );
            assert_eq!(
                lightvgl_sys::LV_DRAW_SW_SUPPORT_L8,
                1,
                "LV_DRAW_SW_SUPPORT_L8 must be set to 1"
            );
        }
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_I1 => {
            assert_eq!(
                lightvgl_sys::LV_COLOR_DEPTH,
                1,
                "LV_COLOR_DEPTH must be set to 1"
            );
            assert_eq!(
                lightvgl_sys::LV_DRAW_SW_SUPPORT_I1,
                1,
                "LV_DRAW_SW_SUPPORT_I1 must be set to 1"
            );
        }
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB565 => {
            assert_eq!(
                lightvgl_sys::LV_COLOR_DEPTH,
                16,
                "LV_COLOR_DEPTH must be set to 16"
            );
            assert_eq!(
                lightvgl_sys::LV_DRAW_SW_SUPPORT_RGB565,
                1,
                "LV_DRAW_SW_SUPPORT_RGB565 must be set to 1"
            );
        }
        lightvgl_sys::lv_color_format_t_LV_COLOR_FORMAT_RGB888 => {
            assert_eq!(
                lightvgl_sys::LV_COLOR_DEPTH,
                24,
                "LV_COLOR_DEPTH must be set to 24"
            );
            assert_eq!(
                lightvgl_sys::LV_DRAW_SW_SUPPORT_RGB888,
                1,
                "LV_DRAW_SW_SUPPORT_RGB888 must be set to 1"
            );
        }
        _ => unreachable!("unsupported color format"),
    }
}

/// Represents a sub-area of the display that is being updated.
pub struct Area {
    pub x1: i16,
    pub x2: i16,
    pub y1: i16,
    pub y2: i16,
}

/// An update to the display information, contains the area that is being
/// updated and the color of the pixels that need to be updated. The colors
/// are represented in a contiguous array.
pub struct DisplayRefresh<'a, const N: usize, C> {
    pub rectangle: Rectangle,
    pub colors: &'a [C],
    pub display: Display,
}

#[expect(clippy::arithmetic_side_effects)]
unsafe extern "C" fn disp_flush_trampoline<F, const N: usize, C>(
    display: *mut lightvgl_sys::lv_display_t,
    area: *const lightvgl_sys::lv_area_t,
    color_p: *mut u8,
) where
    F: FnMut(&mut DisplayRefresh<N, C>),
{
    unsafe {
        let user_data = lv_display_get_user_data(display);
        if !user_data.is_null() {
            let callback = &mut *(user_data.cast::<F>());

            let buf = color_p.cast::<C>();

            let w = ((*area).x2 - (*area).x1 + 1).cast_unsigned();
            let h = ((*area).y2 - (*area).y1 + 1).cast_unsigned();
            let rectangle = Rectangle {
                size: Size {
                    width: w,
                    height: h,
                },
                top_left: Point {
                    x: (*area).x1,
                    y: (*area).y1,
                },
            };

            let slice = ::core::slice::from_raw_parts(buf, (w * h) as usize);

            let mut update = DisplayRefresh {
                rectangle,
                colors: slice,
                display: Display::from_ptr_unchecked(display),
            };
            callback(&mut update);
            lv_display_flush_ready(display);
        } else {
            crate::warn!("Display callback user data was null, this should never happen!");
        }
    }
}

impl<const N: usize, C> DisplayRefresh<'_, N, C> {
    #[expect(clippy::arithmetic_side_effects)]
    pub fn as_pixels<PC>(&self) -> impl IntoIterator<Item = Pixel<PC>>
    where
        C: Clone,
        PC: PixelColor + From<C>,
    {
        let area = &self.rectangle;
        let top_left = area.top_left;
        let Point { x: x1, y: y1 } = top_left;
        let bottom_right = area.bottom_right().unwrap();
        let x2 = bottom_right.x;

        let mut ix = x1;
        let mut iy = y1;
        self.colors.iter().map(move |raw_color| {
            if ix > x2 {
                ix = x1;
                iy += 1;
            }
            Pixel(Point::new(ix, iy), raw_color.to_owned().into())
        })
    }
}

pub struct DrawBuffer<const N: usize, C: LvglColorFormat> {
    raw: NonNull<lv_draw_buf_t>,
    color_depth: PhantomData<C>,
}

impl<const N: usize, C: LvglColorFormat> DrawBuffer<N, C> {
    pub fn new(w: usize, h: usize) -> Self {
        #[expect(clippy::arithmetic_side_effects)]
        {
            assert_eq!(w * h, N);
        }

        let cf = C::as_lv_color_format_t();
        unsafe {
            let raw = NonNull::new(lightvgl_sys::lv_draw_buf_create(
                w.try_into().unwrap(),
                h.try_into().unwrap(),
                cf,
                0,
            ))
            .unwrap();
            Self {
                raw,
                color_depth: PhantomData,
            }
        }
    }

    #[inline]
    pub fn from_raw(raw: NonNull<lv_draw_buf_t>) -> Self {
        Self {
            raw,
            color_depth: PhantomData,
        }
    }

    #[inline]
    pub fn raw(&mut self) -> *const lv_draw_buf_t {
        self.raw.as_ptr().cast_const()
    }

    #[inline]
    pub fn raw_mut(&mut self) -> *mut lv_draw_buf_t {
        self.raw.as_ptr()
    }
}

/// Using a macro because #\[cfg(doctest)\] does not work as expected
/// and this snippet requires embedded_graphics_simulator
///
/// <https://github.com/rust-lang/rust/issues/67295>
#[macro_export]
macro_rules! setup_test_display {
    () => {
        use embedded_graphics::draw_target::DrawTarget;
        use embedded_graphics::pixelcolor::Rgb565;
        use embedded_graphics::prelude::{Point, Size};
        use embedded_graphics_simulator::SimulatorDisplay;
        use lv_bevy_ecs::display::{Display, DrawBuffer};

        const HOR_RES: usize = 320;
        const VER_RES: usize = 240;
        const LINE_HEIGHT: usize = 16;

        let mut sim_display: SimulatorDisplay<Rgb565> =
            SimulatorDisplay::new(Size::new(HOR_RES as u32, VER_RES as u32));

        lv_bevy_ecs::functions::lv_init();

        let mut display = Display::new(HOR_RES, VER_RES);

        let buffer = DrawBuffer::<{ HOR_RES * LINE_HEIGHT }, Rgb565>::new(HOR_RES, LINE_HEIGHT);

        display.register(buffer, move |refresh| {
            //sim_display.draw_iter(refresh.as_pixels()).unwrap();
            sim_display
                .fill_contiguous(&refresh.rectangle, refresh.colors.iter().cloned())
                .unwrap();
        });
    };
}
