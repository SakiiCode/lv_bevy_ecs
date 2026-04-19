//! # Display
//!
//! The `embedded-graphics` crate is used to drive the screens. Examples use `embedded-graphics-simulator` to try them on PC.
//!
//! ## Simulator
//! ```
//! # use embedded_graphics::pixelcolor::Rgb565;
//! # use embedded_graphics::prelude::*;
//! # use embedded_graphics_simulator::*;
//! # use lv_bevy_ecs::display::{DrawBuf, Display};
//! # use lv_bevy_ecs::support::LvglColorFormat;
//! # use lv_bevy_ecs::sys::*;
//! #
//! lv_bevy_ecs::functions::lv_init();
//! const HOR_RES: u32 = 800;
//! const VER_RES: u32 = 480;
//! const LINE_HEIGHT: u32 = 10;
//!
//! let mut sim_display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));
//! let mut display = Display::new(HOR_RES as i32, VER_RES as i32);
//!
//! let buffer = DrawBuf::<{ (HOR_RES * LINE_HEIGHT) as usize }, Rgb565>::new(HOR_RES, LINE_HEIGHT);
//! display.register(buffer, |refresh| {
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

use embedded_graphics::{
    Pixel,
    prelude::{PixelColor, Point, Size},
    primitives::Rectangle,
};
use lightvgl_sys::{lv_color_format_t, lv_display_get_user_data, lv_display_t, lv_draw_buf_t};

use crate::support::LvglColorFormat;

pub struct Display {
    raw: NonNull<lv_display_t>,
}

impl Display {
    pub fn new(hor_res: i32, ver_res: i32) -> Self {
        crate::support::assert_lv_is_initialized!();
        unsafe {
            let raw = NonNull::new(lightvgl_sys::lv_display_create(hor_res, ver_res)).unwrap();
            Self { raw }
        }
    }

    pub fn register<'a, F, const N: usize, C: LvglColorFormat>(
        &'a mut self,
        buffer: DrawBuf<N, C>,
        callback: F,
    ) where
        F: FnMut(&mut DisplayRefresh<N, C>) + 'a,
    {
        let cf = C::as_lv_color_format_t();
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
        unsafe {
            lightvgl_sys::lv_display_set_draw_buffers(
                self.raw_mut(),
                buffer.raw.as_ptr(),
                ::core::ptr::null_mut(),
            );
            register_display(self.raw.as_ptr(), callback);
        }
        crate::info!("Display Registered");
    }

    pub fn flush_is_last(&mut self) -> bool {
        unsafe { lightvgl_sys::lv_display_flush_is_last(self.raw_mut()) }
    }

    pub fn get_default() -> Self {
        unsafe {
            Self {
                raw: NonNull::new(lightvgl_sys::lv_display_get_default()).unwrap(),
            }
        }
    }

    pub fn get_horizontal_resolution(&self) -> i32 {
        unsafe { lightvgl_sys::lv_display_get_horizontal_resolution(self.raw()) }
    }

    pub fn get_vertical_resolution(&self) -> i32 {
        unsafe { lightvgl_sys::lv_display_get_vertical_resolution(self.raw()) }
    }

    pub fn get_color_format(&mut self) -> lv_color_format_t {
        unsafe { lightvgl_sys::lv_display_get_color_format(self.raw_mut()) }
    }

    pub fn raw(&self) -> *const lv_display_t {
        self.raw.as_ptr().cast_const()
    }

    pub fn raw_mut(&mut self) -> *mut lv_display_t {
        self.raw.as_ptr()
    }

    pub fn from_ptr(ptr: *mut lv_display_t) -> Self {
        Self {
            raw: NonNull::new(ptr).unwrap(),
        }
    }

    pub unsafe fn from_ptr_unchecked(ptr: *mut lv_display_t) -> Self {
        unsafe {
            Self {
                raw: NonNull::new_unchecked(ptr),
            }
        }
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

unsafe fn register_display<F, const N: usize, C>(display: *mut lv_display_t, callback: F)
where
    F: FnMut(&mut DisplayRefresh<N, C>),
{
    unsafe {
        lightvgl_sys::lv_display_set_flush_cb(display, Some(disp_flush_trampoline::<F, N, C>));
        lightvgl_sys::lv_display_set_user_data(display, Box::into_raw(Box::new(callback)).cast());
    }
}

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

            let w = (*area).x2 - (*area).x1 + 1;
            let h = (*area).y2 - (*area).y1 + 1;
            let rectangle = Rectangle {
                size: Size {
                    width: w as u32,
                    height: h as u32,
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
        } else {
            crate::warn!("Display callback user data was null, this should never happen!");
        }
        // Indicate to LVGL that we are ready with the flushing
        lightvgl_sys::lv_display_flush_ready(display);
    }
}

impl<const N: usize, C> DisplayRefresh<'_, N, C> {
    pub fn as_pixels<PC>(&self) -> impl IntoIterator<Item = Pixel<PC>>
    where
        C: Clone,
        PC: PixelColor + From<C>,
    {
        let area = &self.rectangle;
        let top_left = area.top_left;
        let Point { x: x1, y: y1 } = top_left;
        let bottom_right = area.bottom_right().unwrap();
        let Point { x: x2, y: y2 } = bottom_right;

        let ys = y1..=y2;
        let xs = (x1..=x2).enumerate();
        let x_len = (x2 - x1 + 1) as usize;

        // We use iterators here to ensure that the Rust compiler can apply all possible
        // optimizations at compile time.
        ys.enumerate().flat_map(move |(iy, y)| {
            xs.clone().map(move |(ix, x)| {
                let color_len = x_len * iy + ix;
                let raw_color = self.colors[color_len].clone();
                let color: PC = raw_color.into();
                Pixel(Point::new(x, y), color)
            })
        })
    }
}

pub struct DrawBuf<const N: usize, C: LvglColorFormat> {
    raw: NonNull<lv_draw_buf_t>,
    color_depth: PhantomData<C>,
}

impl<const N: usize, C: LvglColorFormat> DrawBuf<N, C> {
    pub fn new(w: u32, h: u32) -> Self {
        assert_eq!(w * h, N as u32);
        let cf = C::as_lv_color_format_t();
        unsafe {
            let raw = NonNull::new(lightvgl_sys::lv_draw_buf_create(w, h, cf, 0)).unwrap();
            Self {
                raw,
                color_depth: PhantomData,
            }
        }
    }
    pub fn raw(&self) -> *mut lv_draw_buf_t {
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

        const HOR_RES: u32 = 320;
        const VER_RES: u32 = 240;
        const LINE_HEIGHT: u32 = 16;

        let mut sim_display: SimulatorDisplay<Rgb565> =
            SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

        lv_bevy_ecs::functions::lv_init();

        let mut display = Display::new(HOR_RES as i32, VER_RES as i32);

        let buffer =
            DrawBuf::<{ (HOR_RES * LINE_HEIGHT) as usize }, Rgb565>::new(HOR_RES, LINE_HEIGHT);

        display.register(buffer, |refresh| {
            //sim_display.draw_iter(refresh.as_pixels()).unwrap();
            sim_display
                .fill_contiguous(&refresh.rectangle, refresh.colors.iter().cloned())
                .unwrap();
        });
    };
}
