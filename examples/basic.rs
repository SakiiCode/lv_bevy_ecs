use std::{
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

use bevy_ecs::{schedule::Schedule, world::World};
use lv_bevy_ecs::{animation::Animation, LvError};

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

use lvgl_sys::{__uint8_t, lv_align_t_LV_ALIGN_CENTER, lv_area_t, lv_display_create, lv_display_set_flush_cb, lv_display_t, LV_OPA_0, LV_OPA_100, LV_OPA_50, LV_PART_MAIN};

fn main() -> Result<(), LvError> {
    let mut world = World::new();
    world.add_observer(on_insert_children);

    const HOR_RES: u32 = 240;
    const VER_RES: u32 = 240;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Button Example", &output_settings);

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();

    let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
        sim_display.draw_iter(refresh.as_pixels()).unwrap();
    })?;

    // Define the initial state of your input
    let mut latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = Pointer::register(|| latest_touch_status, &display)?;

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

    loop {
        let start = Instant::now();

        /*window.update(&sim_display);
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
                }
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => {
                    latest_touch_status = PointerInputData::Touch(point).released().once();
                }
                SimulatorEvent::Quit => exit(0),
                _ => {}
            }
        }*/
        // Run the schedule once. If your app has a "loop", you would run this once per loop
        schedule.run(&mut world);
        unsafe {
            lvgl_sys::lv_timer_handler();
            sleep(Duration::from_millis(5));

            lvgl_sys::lv_tick_inc(Instant::now().duration_since(start).as_millis() as u32);
        }
    }
}

extern "C" fn flush_cb(display: *mut lv_display_t, area: *const lv_area_t, px_map: *mut u8){
            

}