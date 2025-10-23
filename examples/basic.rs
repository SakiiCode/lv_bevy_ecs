use std::{
    process::exit,
    time::{Duration, Instant},
};

use lv_bevy_ecs::{
    animation::Animation,
    bevy::{component::Component, entity::Entity, query::With},
    display::{Display, DrawBuffer},
    error,
    events::Event,
    functions::*,
    info,
    input::{BufferStatus, InputDevice, InputEvent, InputState, Pointer},
    styles::Style,
    support::{Align, LvError, OpacityLevel},
    sys::lv_part_t_LV_PART_MAIN,
    widgets::{Arc, Button, Label, LvglWorld},
};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::{Point, Size},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

#[derive(Component)]
struct DynamicButton;

fn main() -> Result<(), LvError> {
    lv_log_init();
    // to use an other logging backend, simply initialize it instead of lv_log_init()
    // env_logger::init();

    const HOR_RES: u32 = 320;
    const VER_RES: u32 = 240;
    const LINE_HEIGHT: u32 = 16;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Button Example", &output_settings);

    info!("SIMULATOR OK");
    error!("Random error");

    let mut display = Display::create(HOR_RES as i32, VER_RES as i32);

    let buffer =
        DrawBuffer::<{ (HOR_RES * LINE_HEIGHT) as usize }, Rgb565>::create(HOR_RES, LINE_HEIGHT);

    info!("Display OK");

    display.register(buffer, |refresh| {
        //sim_display.draw_iter(refresh.as_pixels()).unwrap();
        sim_display
            .fill_contiguous(&refresh.rectangle, refresh.colors.iter().cloned())
            .unwrap();
    });

    info!("Display Driver OK");

    // Define the initial state of your input
    //let mut latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();
    let mut latest_touch_status = InputEvent {
        status: BufferStatus::Once,
        state: InputState::Released,
        data: Point::new(0, 0),
    };

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = InputDevice::<Pointer>::create(|| latest_touch_status);

    info!("Input OK");

    let mut world = LvglWorld::new();

    info!("ECS OK");

    {
        let mut button = Button::create_widget();
        let mut label = Label::create_widget();
        lv_label_set_text(&mut label, c"SPAWN");
        //lv_obj_align(&mut button, LV_ALIGN_CENTER as u8, 10, 10);
        let label_entity = world.spawn((Label, label)).id();

        let anim = Animation::new(
            Duration::from_secs(5),
            OpacityLevel::Transparent as i32,
            OpacityLevel::Cover as i32,
            |obj, val| {
                lv_obj_set_style_opa(obj, val as u8, lv_part_t_LV_PART_MAIN);
            },
        );

        lv_obj_add_event_cb(&mut button, Event::Clicked, |_| {
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
                    let mut dynamic_button = Button::create_widget();
                    let mut label = Label::create_widget();
                    lv_obj_set_align(&mut dynamic_button, Align::TopRight.into());
                    lv_label_set_text(&mut label, c"This is dynamic");
                    world
                        .spawn((DynamicButton, Button, dynamic_button))
                        .with_child((Label, label));
                }
            }
        });

        let mut button_entity = world.spawn((Button, button, anim));

        button_entity.add_child(label_entity);

        let mut style = Style::default();
        lv_style_set_opa(&mut style, OpacityLevel::Percent50 as u8);
        lv_style_set_align(&mut style, Align::TopLeft.into());
        lv_style_set_bg_color(&mut style, lv_color_make(255, 0, 0));

        button_entity.insert(style);
        //button_entity.remove::<Style>();
        // button_entity.insert(style);

        let mut arc = Arc::create_widget();
        lv_obj_set_align(&mut arc, Align::BottomMid.into());

        world.spawn((Arc, arc));
    }

    info!("Create OK");

    let mut is_pointer_down = false;

    let mut prev_time = Instant::now();

    window.update(&sim_display);

    loop {
        let current_time = Instant::now();
        let diff = current_time.duration_since(prev_time);
        prev_time = current_time;

        let events = window.events().peekable();

        for event in events {
            #[allow(unused_assignments)]
            match event {
                SimulatorEvent::MouseButtonDown {
                    mouse_btn: _,
                    point,
                } => {
                    info!("Clicked on: {:?}", point);
                    //latest_touch_status = PointerInputData::Touch(point).pressed().once();
                    latest_touch_status = InputEvent {
                        status: BufferStatus::Once,
                        state: InputState::Pressed,
                        data: point,
                    };
                    is_pointer_down = true;
                }
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => {
                    //latest_touch_status = PointerInputData::Touch(point).released().once();
                    latest_touch_status = InputEvent {
                        status: BufferStatus::Once,
                        state: InputState::Released,
                        data: point,
                    };
                    is_pointer_down = false;
                }
                SimulatorEvent::MouseMove { point } => {
                    if is_pointer_down {
                        //latest_touch_status = PointerInputData::Touch(point).pressed().once();
                        latest_touch_status = InputEvent {
                            status: BufferStatus::Once,
                            state: InputState::Pressed,
                            data: point,
                        };
                    }
                }
                SimulatorEvent::Quit => exit(0),
                _ => {}
            }
        }

        lv_tick_inc(diff);

        lv_timer_handler();

        window.update(&sim_display);
    }
}
