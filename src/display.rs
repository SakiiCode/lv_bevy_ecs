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
//! const HOR_RES: u32 = 800;
//! const VER_RES: u32 = 480;
//! const LINE_HEIGHT: u32 = 10;
//!
//! let mut sim_display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));
//! let mut display = Display::create(HOR_RES as i32, VER_RES as i32);
//!
//! let buffer = DrawBuffer::<{ (HOR_RES * LINE_HEIGHT) as usize }, Rgb565>::create(HOR_RES, LINE_HEIGHT);
//! display.register(buffer, |refresh| {
//!     // alternative (slower): sim_display.draw_iter(refresh.as_pixels()).unwrap();
//!     sim_display
//!         .fill_contiguous(&refresh.rectangle, refresh.colors.iter().cloned())
//!         .unwrap();
//! });
//!
//! unsafe {
//!     let default_display = lv_display_get_default();
//!     assert_eq!(lv_display_get_horizontal_resolution(default_display), HOR_RES as i32);
//!     assert_eq!(lv_display_get_vertical_resolution(default_display), VER_RES as i32);
//!     assert_eq!(lv_display_get_color_format(default_display), Rgb565::as_lv_color_format_t());
//! }
//! ```
//!
//! ## ESP32
//!
//! For code example on how to create a display on embedded,
//! check out [lvgl-bevy-demo](https://github.com/SakiiCode/lvgl-bevy-demo).

use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};

use embedded_graphics::{
    Pixel,
    prelude::{PixelColor, Point, Size},
    primitives::Rectangle,
};
use lightvgl_sys::{
    lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_PARTIAL, lv_display_t, lv_draw_buf_t,
};

use crate::{info, support::LvglColorFormat, warn};

pub struct Display {
    raw: NonNull<lv_display_t>,
}

impl Display {
    pub fn create(hor_res: i32, ver_res: i32) -> Self {
        unsafe {
            let raw = NonNull::new(lightvgl_sys::lv_display_create(hor_res, ver_res)).unwrap();
            Self { raw }
        }
    }

    pub fn register<F, const N: usize, C: LvglColorFormat>(
        &mut self,
        buffer: DrawBuffer<N, C>,
        callback: F,
    ) where
        F: FnMut(&mut DisplayRefresh<N, C>),
    {
        unsafe {
            lightvgl_sys::lv_display_set_draw_buffers(
                self.raw(),
                buffer.raw.as_ptr(),
                std::ptr::null_mut(),
            );
            register_display(self.raw.as_ptr(), callback);
        }
        info!("Display Registered");
    }

    pub fn raw(&self) -> *mut lv_display_t {
        self.raw.as_ptr()
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
}

unsafe fn register_display<F, const N: usize, C>(display: *mut lv_display_t, callback: F)
where
    F: FnMut(&mut DisplayRefresh<N, C>),
{
    unsafe {
        lightvgl_sys::lv_display_set_flush_cb(display, Some(disp_flush_trampoline::<F, N, C>));
        lightvgl_sys::lv_display_set_user_data(
            display,
            Box::into_raw(Box::new(callback)) as *mut _ as *mut c_void,
        );
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
        let display_driver = *display;
        if !display_driver.user_data.is_null() {
            let callback = &mut *(display_driver.user_data as *mut F);

            let buf = color_p as *mut C;

            let w = (*area).x2 - (*area).x1 + 1;
            let h = (*area).y2 - (*area).y1 + 1;
            let rectangle = Rectangle {
                size: Size {
                    width: w as u32,
                    height: h as u32,
                },
                top_left: Point {
                    x: (*area).x1.into(),
                    y: (*area).y1.into(),
                },
            };

            let slice = std::slice::from_raw_parts(buf, (w * h) as usize);

            let mut update = DisplayRefresh {
                rectangle,
                colors: slice,
            };
            callback(&mut update);
        } else {
            warn!("Display callback user data was null, this should never happen!");
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
                Pixel(Point::new(x as i32, y as i32), color)
            })
        })
    }
}

pub struct DrawBuffer<const N: usize, C: LvglColorFormat> {
    raw: NonNull<lv_draw_buf_t>,
    color_depth: PhantomData<C>,
}

impl<const N: usize, C: LvglColorFormat> DrawBuffer<N, C> {
    pub fn create(w: u32, h: u32) -> Self {
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

/// Using a macro because #\[cfg(doctest)\] does not work properly
///
/// <https://github.com/rust-lang/rust/issues/67295>
///
/// Intended for doctest use only!
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

        let mut display = Display::create(HOR_RES as i32, VER_RES as i32);

        let buffer = DrawBuffer::<{ (HOR_RES * LINE_HEIGHT) as usize }, Rgb565>::create(
            HOR_RES,
            LINE_HEIGHT,
        );

        display.register(buffer, |refresh| {
            //sim_display.draw_iter(refresh.as_pixels()).unwrap();
            sim_display
                .fill_contiguous(&refresh.rectangle, refresh.colors.iter().cloned())
                .unwrap();
        });
    };
}
