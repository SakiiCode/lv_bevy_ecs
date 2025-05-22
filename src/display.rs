use std::ptr::NonNull;

use cty::c_void;
use embedded_graphics::{
    Pixel,
    prelude::{PixelColor, Point},
};
use lvgl_sys::{
    lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_PARTIAL, lv_display_t, lv_draw_buf_t,
};

use crate::support::Color;

pub struct Display {
    raw: NonNull<lv_display_t>,
}

impl Display {
    pub fn create(hor_res: i32, ver_res: i32) -> Self {
        unsafe {
            let raw = NonNull::new(lvgl_sys::lv_display_create(hor_res, ver_res)).unwrap();
            Self { raw }
        }
    }

    pub fn register<F, const N: usize>(&mut self, buffer: DrawBuffer<N>, callback: F)
    where
        F: FnMut(&DisplayRefresh<N>),
    {
        unsafe {
            lvgl_sys::lv_display_set_buffers(
                self.raw(),
                buffer.raw.as_ptr() as *mut c_void,
                std::ptr::null_mut(),
                N as u32,
                lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_PARTIAL,
            );
            register_display(self.raw.as_ptr(), callback);
        }
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
pub struct DisplayRefresh<const N: usize> {
    pub area: Area,
    pub colors: *mut u16,
}

unsafe fn register_display<F, const N: usize>(display: *mut lv_display_t, callback: F)
where
    F: FnMut(&DisplayRefresh<N>),
{
    unsafe {
        lvgl_sys::lv_display_set_flush_cb(display, Some(disp_flush_trampoline::<F, N>));
        println!("Callback OK");
        lvgl_sys::lv_display_set_user_data(
            display,
            Box::into_raw(Box::new(callback)) as *mut _ as *mut c_void,
        );
    }
}

unsafe extern "C" fn disp_flush_trampoline<'a, F, const N: usize>(
    display: *mut lvgl_sys::lv_display_t,
    area: *const lvgl_sys::lv_area_t,
    color_p: *mut u8,
) where
    F: FnMut(&DisplayRefresh<N>) + 'a,
{
    unsafe {
        let display_driver = *display;
        if !display_driver.user_data.is_null() {
            let callback = &mut *(display_driver.user_data as *mut F);

            //let mut colors = Box::new([Color::default(); N]);
            let buf16 = color_p as *mut u16;
            //lvgl_sys::lv_draw_sw_rgb565_swap(buf16 as *mut c_void, (N/2) as u32);
            /*for (color_len, color) in colors.iter_mut().enumerate() {
                let lv_color = buf16.add(color_len);

                let r = (*lv_color >> 11) & 0x1F;
                let g = (*lv_color >> 5) & 0x3F;
                let b = *lv_color & 0x1F;

                *color = Color::from_rgb((r as u8, g as u8, b as u8));
            }*/

            let update = DisplayRefresh {
                area: Area {
                    x1: (*area).x1 as i16,
                    x2: (*area).x2 as i16,
                    y1: (*area).y1 as i16,
                    y2: (*area).y2 as i16,
                },
                colors: buf16,
            };
            callback(&update);
        } else {
            println!("User data is null");
        }
        // Not doing this causes a segfault in rust >= 1.69.0
        //*disp_drv = display_driver;
        // Indicate to LVGL that we are ready with the flushing
        lvgl_sys::lv_display_flush_ready(display);
    }
}

/*impl<const N: usize> DisplayRefresh<N> {
    pub fn as_pixels<C>(&self) -> impl IntoIterator<Item = Pixel<C>> + '_
    where
        C: PixelColor + From<Color>,
    {
        let area = &self.area;
        let x1 = area.x1;
        let x2 = area.x2;
        let y1 = area.y1;
        let y2 = area.y2;

        let ys = y1..=y2;
        let xs = (x1..=x2).enumerate();
        let x_len = (x2 - x1 + 1) as usize;

        // We use iterators here to ensure that the Rust compiler can apply all possible
        // optimizations at compile time.
        ys.enumerate().flat_map(move |(iy, y)| {
            xs.clone().map(move |(ix, x)| {
                let color_len = x_len * iy + ix;
                let raw_color = self.colors[color_len];
                Pixel(Point::new(x as i32, y as i32), raw_color.into())
            })
        })
    }
}*/

pub struct DrawBuffer<const N: usize> {
    raw: NonNull<lv_draw_buf_t>,
}

impl<const N: usize> DrawBuffer<N> {
    pub fn create(w: u32, h: u32, cf: lvgl_sys::lv_color_format_t) -> Self {
        assert_eq!(w * h, N as u32);
        unsafe {
            let raw = NonNull::new(lvgl_sys::lv_draw_buf_create(w, h, cf, 0)).unwrap();
            Self { raw }
        }
    }
    pub fn raw(&self) -> *mut lv_draw_buf_t {
        self.raw.as_ptr()
    }
}
