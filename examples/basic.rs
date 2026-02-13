use std::{
    process::exit,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
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
    support::{Align, OpacityLevel},
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
#[component(storage = "SparseSet")]
struct DynamicButton;

fn main() {
    lv_bevy_ecs::logging::lv_log_init();

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

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = InputDevice::<Pointer>::create(|| get_touch_input(window.events()));

    info!("Input OK");

    lv_tick_set_cb(|| {
        let current_time = SystemTime::now();
        let diff = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time should only go forward");
        let ms = diff.as_millis() as u32;
        ms
    });

    let mut world = LvglWorld::default();

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

    window.update(&sim_display);

    loop {
        lv_timer_handler();

        window.update(&sim_display);
    }
}

fn get_touch_input(events: impl Iterator<Item = SimulatorEvent>) -> InputEvent<Pointer> {
    static IS_POINTER_DOWN: AtomicBool = AtomicBool::new(false);

    static LATEST_TOUCH_STATUS: Mutex<InputEvent<Pointer>> =
        Mutex::new(InputEvent::new(Point::zero()));

    let mut next_touch_status = None;

    for event in events {
        match event {
            SimulatorEvent::MouseButtonDown {
                mouse_btn: _,
                point,
            } => {
                next_touch_status = Some(InputEvent {
                    status: BufferStatus::Once,
                    state: InputState::Pressed,
                    data: point,
                });
                IS_POINTER_DOWN.store(true, Ordering::Relaxed);
            }
            SimulatorEvent::MouseButtonUp {
                mouse_btn: _,
                point,
            } => {
                next_touch_status = Some(InputEvent {
                    status: BufferStatus::Once,
                    state: InputState::Released,
                    data: point,
                });
                IS_POINTER_DOWN.store(false, Ordering::Relaxed);
            }
            SimulatorEvent::MouseMove { point } => {
                if IS_POINTER_DOWN.load(Ordering::Relaxed) {
                    next_touch_status = Some(InputEvent {
                        status: BufferStatus::Once,
                        state: InputState::Pressed,
                        data: point,
                    });
                }
            }
            SimulatorEvent::Quit => exit(0),
            _ => {}
        }
    }

    let mut lock = LATEST_TOUCH_STATUS.lock().unwrap();

    if let Some(latest_touch_status) = next_touch_status {
        *lock = latest_touch_status;
    }
    return *lock;
}
