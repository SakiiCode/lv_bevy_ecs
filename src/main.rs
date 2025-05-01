use std::{
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

use animation::Animation;
use bevy_ecs::{entity::Entity, schedule::Schedule, world::World};

use cstr_core::cstr;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::{Point, Size},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use generated::lv_label_set_text;
use lvgl::{
    Display, DrawBuffer, LvError,
    input_device::{
        InputDriver,
        pointer::{Pointer, PointerInputData},
    },
};
use lvgl_sys::{LV_ALIGN_CENTER, LV_OPA_0, LV_OPA_50, LV_OPA_100, LV_PART_MAIN};
use styles::Style;
use widgets::{Button, Label, on_insert_children};

mod animation;
#[allow(dead_code)]
mod generated;
mod styles;
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

    //world.insert_resource(DisplayResource(sim_display));

    // Define the initial state of your input
    let mut latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();

    //world.insert_resource(TouchStatus(latest_touch_status));

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = Pointer::register(|| latest_touch_status, &display);

    // Create screen and widgets
    //let screen = display.get_scr_act()?;
    {
        /*unsafe {
            //let button_entity = ButtonComponent::spawn_entity(None, &mut world)?;
            let button_widget = ButtonComponent::new_widget(None, &mut world)?;
            //let btn_raw = Widget::get(button_entity, &world).obj.raw;
            let btn_raw = button_widget.obj.raw;
            let button_entity = world.spawn((button_widget, ButtonComponent)).id();
            lv_obj_align(btn_raw, LV_ALIGN_LEFT_MID as u8, 30, 0);
            lv_obj_set_size(btn_raw, 180, 80);
            //let _btn_lbl = LabelComponent::spawn_entity(Some(button_entity), &mut world)?;
            let label_widget = LabelComponent::new_widget(Some(button_entity), &mut world)?;
            let lbl_raw = label_widget.obj.raw;
            lv_label_set_text(
                lbl_raw,
                CString::new("Click me!").unwrap().as_ptr(),
            );
            world
                .entity_mut(button_entity)
                .with_child((label_widget, LabelComponent));
            //button_entity.with_child((btn_lbl, LabelComponent));
            //world.spawn((button, ButtonComponent)).with_child((btn_lbl, LabelComponent));
        }*/

        //let button = Button::create_widget()?;
        //let label = Label::create_widget()?;

        let button = Button::create_widget()?;
        let mut label = Label::create_widget()?;
        lv_label_set_text(&mut label, cstr!("OKE'SOS"));
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
            lvgl_sys::lv_style_set_align(style.raw.as_mut(), LV_ALIGN_CENTER as u8);
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
                    /*let mut eids = Vec::new();
                    let mut widgets = world.query::<Entity>();
                    for entity_id in widgets.iter(&world) {
                        eids.push(entity_id);
                    }
                    for eid in eids {
                        dbg!(&eid);
                        //let _ = widgets.get(eid).and_then(|w|Ok(w.delete()));
                        //widgets.delete(eid);
                        world.despawn(eid);
                    }*/
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
