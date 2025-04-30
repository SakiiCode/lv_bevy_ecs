use std::{
    ffi::CString,
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

use bevy_ecs::{entity::Entity, query::With, resource::Resource, schedule::Schedule, world::World};

use embedded_graphics::{pixelcolor::Rgb565, prelude::{Point, Size}, draw_target::DrawTarget};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl::{
    Display, DrawBuffer, LvError, NativeObject,
    input_device::{
        BufferStatus, InputDriver,
        pointer::{Pointer, PointerInputData},
    },
};
use lvgl_sys::{lv_label_set_text, lv_obj_align, lv_obj_set_size, LV_ALIGN_LEFT_MID};
use widgets::{Button, Label, Widget};

mod widgets;

/*#[derive(Resource)]
struct DisplayResource(SimulatorDisplay<Rgb565>);

#[derive(Resource)]
struct TouchStatus(BufferStatus);*/

/*#[inline(always)]
fn init(world: &mut World) -> LvResult<()> {
    const HOR_RES: u32 = 240;
    const VER_RES: u32 = 240;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let window = Window::new("Button Example", &output_settings);

    world.add_unique_non_send_sync(WindowResource(window));

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();

    let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
        sim_display.draw_iter(refresh.as_pixels()).unwrap();
    })?;

    world.add_unique(DisplayResource(sim_display));

    // Define the initial state of your input
    let latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();

    world.add_unique(TouchStatus(latest_touch_status));

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = Pointer::register(|| latest_touch_status, &display);

    Ok(())
}*/

fn main() -> Result<(), LvError> {
    let mut world = World::new();

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

    //world.insert_resource(DisplayResource(sim_display));

    // Define the initial state of your input
    let mut latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();

    //world.insert_resource(TouchStatus(latest_touch_status));

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = Pointer::register(|| latest_touch_status, &display);

    // Create screen and widgets
    let screen = display.get_scr_act()?;
    {
        unsafe {
            let button = Button::new(screen.raw().as_mut())?;
            let btn_raw = button.obj.raw;
            lv_obj_align(btn_raw, LV_ALIGN_LEFT_MID as u8, 30, 0);
            lv_obj_set_size(btn_raw, 180, 80);
            let btn_lbl = Label::new(btn_raw.as_mut().unwrap())?;
            let lbl_raw = btn_lbl.obj.raw;
            lv_label_set_text(
                lbl_raw,
                CString::new("Click me!").unwrap().as_c_str().as_ptr(),
            );
            world.spawn((button, Button)).with_child((btn_lbl, Label));
        }
    }

    // Create a new Schedule, which defines an execution strategy for Systems
    let mut schedule = Schedule::default();

    // Add our system to the schedule
    //schedule.add_systems(movement);
    //world.add_observer(drop_widget);

    loop {
        let start = Instant::now();

        window.update(&sim_display);
        let events = window.events().peekable();

        for event in events {
            match event {
                SimulatorEvent::MouseButtonDown {
                    mouse_btn: _,
                    point,
                } => {
                    println!("Clicked on: {:?}", point);
                    latest_touch_status = PointerInputData::Touch(point).pressed().once();
                    let mut eids = Vec::new();
                    let mut widgets = world.query_filtered::<Entity, With<Widget>>();
                    for entity_id in widgets.iter(&world) {
                        eids.push(entity_id);
                    }
                    for eid in eids {
                        dbg!(&eid);
                        //let _ = widgets.get(eid).and_then(|w|Ok(w.delete()));
                        //widgets.delete(eid);
                        world.despawn(eid);
                    }
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
        }

        // Run the schedule once. If your app has a "loop", you would run this once per loop
        schedule.run(&mut world);

        lvgl::task_handler();

        sleep(Duration::from_millis(5));
        lvgl::tick_inc(Instant::now().duration_since(start));
    }

    Ok(())
}
