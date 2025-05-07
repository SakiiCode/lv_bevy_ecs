use std::{
    os::raw::c_void, process::exit, thread::sleep, time::{Duration, Instant}
};

use bevy_ecs::{schedule::Schedule, world::World};
use lv_bevy_ecs::{LvError, animation::Animation, support::Color};

use cstr_core::cstr;
use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::Rgb888, prelude::{PixelColor, Point, Size}, Pixel
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lv_bevy_ecs::styles::Style;
use lv_bevy_ecs::widgets::{Button, Label, on_insert_children};

use lvgl_sys::{
    lv_align_t_LV_ALIGN_CENTER, lv_color_format_t_LV_COLOR_FORMAT_RGB565, lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_PARTIAL, lv_display_t, lv_draw_buf_create, LV_OPA_0, LV_OPA_100, LV_OPA_50, LV_PART_MAIN
};

fn main() -> Result<(), LvError> {
    const HOR_RES: u32 = 320;
    const VER_RES: u32 = 240;
    const RES: usize = (HOR_RES * VER_RES) as usize;

    let mut sim_display: SimulatorDisplay<Rgb888> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Button Example", &output_settings);

    println!("SIMULATOR OK");

    unsafe {
        lvgl_sys::lv_init();

        let display = lvgl_sys::lv_display_create(HOR_RES as i32, VER_RES as i32);

        println!("Display OK");
        let update_function = |refresh: &DisplayRefresh<RES>| {
            sim_display.draw_iter(refresh.as_pixels()).unwrap();
        };

        register_display(display, update_function);
        //lvgl_sys::lv_display_set_default(display);

        let buffer = lv_draw_buf_create(
            HOR_RES,
            VER_RES / 30,
            lv_color_format_t_LV_COLOR_FORMAT_RGB565,
            0,
        );

        lvgl_sys::lv_display_set_buffers(
            display,
            buffer as *mut c_void,
            std::ptr::null_mut(),
            HOR_RES * VER_RES / 30,
            lv_display_render_mode_t_LV_DISPLAY_RENDER_MODE_PARTIAL,
        );

        println!("User Data OK");
    }
    println!("INIT OK");

    let mut world = World::new();
    world.add_observer(on_insert_children);

    println!("ECS OK");

    /*
    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();

    let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
        sim_display.draw_iter(refresh.as_pixels()).unwrap();
    })?;

    // Define the initial state of your input
    let mut latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = Pointer::register(|| latest_touch_status, &display)?;*/

    // Create screen and widgets
    //let screen = display.get_scr_act()?;
    {
        let button = Button::create_widget()?;
        let mut label = Label::create_widget()?;
        unsafe {
            lvgl_sys::lv_label_set_text(label.raw().as_ptr(), cstr!("OKE'SOS").as_ptr());
        }
        //lv_obj_align(&mut button, LV_ALIGN_CENTER as u8, 10, 10);
        let label_entity = world.spawn((Label, label)).id();

        let anim = Animation::new(
            Duration::from_secs(5),
            LV_OPA_0 as i32,
            LV_OPA_100 as i32,
            |obj, val| unsafe {
                lvgl_sys::lv_obj_set_style_opa(obj.raw.as_ptr(), val as u8, LV_PART_MAIN);
            },
        );

        let mut button_entity = world.spawn((Button, button, anim));

        button_entity.add_child(label_entity);

        let mut style = Style::default();
        unsafe {
            lvgl_sys::lv_style_set_opa(style.raw.as_mut(), LV_OPA_50 as u8);
            lvgl_sys::lv_style_set_align(style.raw.as_mut(), lv_align_t_LV_ALIGN_CENTER as u32);
            lvgl_sys::lv_style_set_bg_color(style.raw.as_mut(), lvgl_sys::lv_color_make(0, 0, 255));
        }

        button_entity.insert(style);
        //button_entity.remove::<Style>();
        // button_entity.insert(style);
    }

    println!("Create OK");
    // Create a new Schedule, which defines an execution strategy for Systems
    let mut schedule = Schedule::default();

    // Add our system to the schedule
    //schedule.add_systems(movement);
    //world.add_observer(drop_widget);

    //let mut last_tick = SDL_GetTicks();
    loop {
        let start = Instant::now();

        window.update(&sim_display);
        let events = window.events().peekable();

        for event in events {
            #[allow(unused_assignments)]
            match event {
                SimulatorEvent::MouseButtonDown {
                    mouse_btn: _,
                    point,
                } => {
                    println!("Clicked on: {:?}", point);
                    //latest_touch_status = PointerInputData::Touch(point).pressed().once();
                }
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => {
                    //latest_touch_status = PointerInputData::Touch(point).released().once();
                }
                SimulatorEvent::Quit => exit(0),
                _ => {}
            }
        }

        sleep(Duration::from_millis(5));
        // Run the schedule once. If your app has a "loop", you would run this once per loop
        schedule.run(&mut world);
        unsafe {
            lvgl_sys::lv_tick_inc(Instant::now().duration_since(start).as_millis() as u32);

            lvgl_sys::lv_timer_handler();
        }
    }
}

/*
/// An LVGL color. Equivalent to `lv_color_t`.
#[derive(Copy, Clone)]
pub struct Color {
    pub(crate) raw: lvgl_sys::lv_color_t,
}*/

/*impl Into<Rgb565> for Color {
    fn into(self) -> Rgb565 {
        Rgb565::new(self.r(), self.g(), self.b())
    }
}*/

/*impl From<Color> for Rgb565 {
    fn from(value: Color) -> Self {
        Rgb565::new(value.r(), value.g(), value.b())
    }
}*/

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
    pub colors: [Color; N],
}

unsafe fn register_display<F, const N: usize>(display: *mut lv_display_t, callback: F)
where
    F: FnMut(&DisplayRefresh<N>),
{
    lvgl_sys::lv_display_set_flush_cb(display, Some(disp_flush_trampoline::<F, N>));
    println!("Callback OK");
    lvgl_sys::lv_display_set_user_data(
        display,
        Box::into_raw(Box::new(callback)) as *mut _ as *mut c_void,
    );
}

unsafe extern "C" fn disp_flush_trampoline<'a, F, const N: usize>(
    disp_drv: *mut lvgl_sys::lv_display_t,
    area: *const lvgl_sys::lv_area_t,
    color_p: *mut u8,
) where
    F: FnMut(&DisplayRefresh<N>) + 'a,
{
    let display_driver = *disp_drv;
    if !display_driver.user_data.is_null() {
        let callback = &mut *(display_driver.user_data as *mut F);

        let mut colors = [Color::default(); N];
        //let buf16 = color_p as *mut u16;
        //lvgl_sys::lv_draw_sw_rgb565_swap(buf16 as *mut c_void, (N/2) as u32);
        for (color_len, color) in colors.iter_mut().enumerate() {
            let lv_color = (color_p.add(color_len*3));

            //*color = lvgl_sys::lv_color_make//Color::from_rgb((77, 77, 77));
            //let red = /
            let r = *lv_color.add(0);
            let g = *lv_color.add(1);
            let b = *lv_color.add(2);
            
            *color = Color::from_rgb((b as u8,g as u8,r as u8));
            
        }

        let update = DisplayRefresh {
            area: Area {
                x1: (*area).x1 as i16,
                x2: (*area).x2 as i16,
                y1: (*area).y1 as i16,
                y2: (*area).y2 as i16,
            },
            colors,
        };
        callback(&update);
    } else {
        println!("User data is null");
    }
    // Not doing this causes a segfault in rust >= 1.69.0
    *disp_drv = display_driver;
    // Indicate to LVGL that we are ready with the flushing
    lvgl_sys::lv_display_flush_ready(disp_drv);
}

impl<const N: usize> DisplayRefresh<N> {
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
}
