use std::{ffi::CString, marker::PhantomData, process::exit, time::Duration};

use lv_bevy_ecs::{
    LvglSchedule, LvglWorld,
    animation::Animation,
    display::{Display, DrawBuffer},
    events::{Event, lv_obj_add_event_cb},
    functions::{
        lv_color_make, lv_label_set_text, lv_obj_set_align, lv_obj_set_style_opa,
        lv_style_set_align, lv_style_set_bg_color, lv_style_set_opa, lv_timer_handler,
    },
    input::{BufferStatus, InputDevice, InputState, LvglInputEvent},
    support::{Align, LvError},
    widgets::Arc,
};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::{Point, Size},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lv_bevy_ecs::styles::Style;
use lv_bevy_ecs::widgets::{Button, Label};

use lv_bevy_ecs::prelude::{
    LV_OPA_0, LV_OPA_50, LV_OPA_100, LV_PART_MAIN, component::Component, entity::Entity,
    query::With,
};

macro_rules! cstr {
    ($txt:literal) => {
        CString::new($txt).unwrap().as_c_str()
    };
}

#[derive(Component)]
struct DynamicButton;

fn main() -> Result<(), LvError> {
    const HOR_RES: u32 = 320;
    const VER_RES: u32 = 240;
    const LINE_HEIGHT: u32 = 16;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Button Example", &output_settings);

    println!("SIMULATOR OK");

    let mut display = Display::create(HOR_RES as i32, VER_RES as i32);

    let buffer =
        DrawBuffer::<{ (HOR_RES * LINE_HEIGHT) as usize }, Rgb565>::create(HOR_RES, LINE_HEIGHT);

    println!("Display OK");

    display.register(buffer, |refresh| {
        //sim_display.draw_iter(refresh.as_pixels()).unwrap();
        sim_display
            .fill_contiguous(&refresh.rectangle, refresh.colors.iter().cloned())
            .unwrap();
    });

    println!("Display Driver OK");

    // Define the initial state of your input
    //let mut latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();
    let mut latest_touch_status = LvglInputEvent {
        status: lv_bevy_ecs::input::BufferStatus::Once,
        state: lv_bevy_ecs::input::InputState::Released,
        data: Point::new(0, 0),
        device_type: PhantomData,
    };

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = InputDevice::create(|| latest_touch_status);

    println!("Input OK");

    let mut world = LvglWorld::new();

    println!("ECS OK");

    {
        let button = Button::create_widget()?;
        let mut label = Label::create_widget()?;
        lv_label_set_text(&mut label, cstr!("SPAWN"));
        //lv_obj_align(&mut button, LV_ALIGN_CENTER as u8, 10, 10);
        let label_entity = world.spawn((Label, label)).id();

        let anim = Animation::new(
            Duration::from_secs(5),
            LV_OPA_0 as i32,
            LV_OPA_100 as i32,
            |obj, val| {
                lv_obj_set_style_opa(obj, val as u8, LV_PART_MAIN);
            },
        );

        lv_obj_add_event_cb(&button, Event::Clicked, |_| {
            match world
                .query_filtered::<Entity, With<DynamicButton>>()
                .single(&world)
                .ok()
            {
                Some(entity) => {
                    world.despawn(entity);
                    /*let mut entities = Vec::new();
                    for entity in world.query_filtered::<Entity, With<Button>>().iter(&world) {
                        entities.push(entity);
                    }
                    for entity in entities{
                        world.despawn(entity);
                    }*/
                }
                None => {
                    let mut dynamic_button = Button::create_widget().unwrap();
                    let mut label = Label::create_widget().unwrap();
                    lv_obj_set_align(&mut dynamic_button, Align::TopRight.into());
                    lv_label_set_text(&mut label, cstr!("This is dynamic"));
                    world
                        .spawn((DynamicButton, Button, dynamic_button))
                        .with_child((Label, label));
                }
            }
        });

        let mut button_entity = world.spawn((Button, button, anim));

        button_entity.add_child(label_entity);

        let mut style = Style::default();
        lv_style_set_opa(&mut style, LV_OPA_50 as u8);
        lv_style_set_align(&mut style, Align::TopLeft.into());
        lv_style_set_bg_color(&mut style, lv_color_make(255, 0, 0));

        button_entity.insert(style);
        //button_entity.remove::<Style>();
        // button_entity.insert(style);

        let mut arc = Arc::create_widget()?;
        lv_obj_set_align(&mut arc, Align::BottomMid.into());

        world.spawn((Arc, arc));
    }

    println!("Create OK");
    // Create a new Schedule, which defines an execution strategy for Systems
    let mut schedule = LvglSchedule::new();

    let mut is_pointer_down = false;
    loop {
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
                    latest_touch_status = LvglInputEvent {
                        status: BufferStatus::Once,
                        state: InputState::Pressed,
                        data: point,
                        device_type: PhantomData,
                    };
                    is_pointer_down = true;
                }
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => {
                    //latest_touch_status = PointerInputData::Touch(point).released().once();
                    latest_touch_status = LvglInputEvent {
                        status: BufferStatus::Once,
                        state: InputState::Released,
                        data: point,
                        device_type: PhantomData,
                    };
                    is_pointer_down = false;
                }
                SimulatorEvent::MouseMove { point } => {
                    if is_pointer_down {
                        //latest_touch_status = PointerInputData::Touch(point).pressed().once();
                        latest_touch_status = LvglInputEvent {
                            status: BufferStatus::Once,
                            state: InputState::Pressed,
                            data: point,
                            device_type: PhantomData,
                        };
                    }
                }
                SimulatorEvent::Quit => exit(0),
                _ => {}
            }
        }

        // Run the schedule once. If your app has a "loop", you would run this once per loop
        schedule.run(&mut world);

        lv_timer_handler();
    }
}
