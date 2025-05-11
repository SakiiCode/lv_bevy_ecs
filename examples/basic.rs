use std::{
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

use bevy_ecs::{schedule::Schedule, world::World};
use lv_bevy_ecs::{
    animation::Animation,
    display::{Display, DrawBuffer},
    input::{InputDevice, PointerInputData},
    support::LvError,
    widgets::Arc,
};

use cstr_core::cstr;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::{Point, Size},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lv_bevy_ecs::styles::Style;
use lv_bevy_ecs::widgets::{Button, Label, on_insert_children};

use lv_bevy_ecs::prelude::{
    LV_OPA_0, LV_OPA_50, LV_OPA_100, LV_PART_MAIN, lv_align_t_LV_ALIGN_BOTTOM_MID,
    lv_align_t_LV_ALIGN_TOP_MID, lv_color_format_t_LV_COLOR_FORMAT_RGB565,
    lv_indev_type_t_LV_INDEV_TYPE_POINTER,
};

fn main() -> Result<(), LvError> {
    const HOR_RES: u32 = 320;
    const VER_RES: u32 = 240;
    const LINE_HEIGHT: u32 = 10;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Button Example", &output_settings);

    println!("SIMULATOR OK");

    lv_bevy_ecs::init();

    let mut display = Display::create(HOR_RES as i32, VER_RES as i32);

    let buffer = DrawBuffer::<{ (HOR_RES * LINE_HEIGHT) as usize }>::create(
        HOR_RES,
        LINE_HEIGHT,
        lv_color_format_t_LV_COLOR_FORMAT_RGB565,
    );

    println!("Display OK");

    display.register(buffer, |refresh| {
        sim_display.draw_iter(refresh.as_pixels()).unwrap();
    });

    println!("Display Driver OK");

    // Define the initial state of your input
    let mut latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = InputDevice::create(lv_indev_type_t_LV_INDEV_TYPE_POINTER, || {
        latest_touch_status
    });

    println!("Input OK");

    let mut world = World::new();
    world.add_observer(on_insert_children);

    println!("ECS OK");

    {
        let button = Button::create_widget()?;
        let label = Label::create_widget()?;
        unsafe {
            lvgl_sys::lv_label_set_text(label.raw(), cstr!("OKE'SOS").as_ptr());
        }
        //lv_obj_align(&mut button, LV_ALIGN_CENTER as u8, 10, 10);
        let label_entity = world.spawn((Label, label)).id();

        let anim = Animation::new(
            Duration::from_secs(5),
            LV_OPA_0 as i32,
            LV_OPA_100 as i32,
            |obj, val| unsafe {
                lvgl_sys::lv_obj_set_style_opa(obj.raw(), val as u8, LV_PART_MAIN);
            },
        );

        let mut button_entity = world.spawn((Button, button, anim));

        button_entity.add_child(label_entity);

        let mut style = Style::default();
        unsafe {
            lvgl_sys::lv_style_set_opa(style.raw(), LV_OPA_50 as u8);
            lvgl_sys::lv_style_set_align(style.raw(), lv_align_t_LV_ALIGN_TOP_MID as u32);
            lvgl_sys::lv_style_set_bg_color(style.raw(), lvgl_sys::lv_color_make(255, 0, 0));
        }

        button_entity.insert(style);
        //button_entity.remove::<Style>();
        // button_entity.insert(style);

        let arc = Arc::create_widget()?;
        unsafe {
            lvgl_sys::lv_obj_set_align(arc.raw(), lv_align_t_LV_ALIGN_BOTTOM_MID);
        }

        world.spawn((Arc, arc));
    }

    println!("Create OK");
    // Create a new Schedule, which defines an execution strategy for Systems
    let mut schedule = Schedule::default();

    let mut is_pointer_down = false;

    let mut prev_time = Instant::now();
    sleep(Duration::from_millis(5));
    loop {
        let current_time = Instant::now();
        let diff = current_time.duration_since(prev_time).as_millis() as u32;
        prev_time = current_time;

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
                    latest_touch_status = PointerInputData::Touch(point).pressed().once();
                    is_pointer_down = true;
                }
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => {
                    latest_touch_status = PointerInputData::Touch(point).released().once();
                    is_pointer_down = false;
                }
                SimulatorEvent::MouseMove { point } => {
                    if is_pointer_down {
                        latest_touch_status = PointerInputData::Touch(point).pressed().once();
                    }
                }
                SimulatorEvent::Quit => exit(0),
                _ => {}
            }
        }

        // Run the schedule once. If your app has a "loop", you would run this once per loop
        schedule.run(&mut world);

        unsafe {
            lvgl_sys::lv_tick_inc(diff);

            lvgl_sys::lv_timer_handler();
        }

        sleep(Duration::from_millis(5));
    }
}
