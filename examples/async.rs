use std::{
    process::exit,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::{Point, Size},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
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
    sys::{lv_part_t_LV_PART_MAIN, lv_style_selector_t},
    widgets::{Arc, Button, Label, LvglWorld},
};
use macro_rules_attribute::apply;
use smol::{Timer, future::yield_now};
use smol_macros::main;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct DynamicButton;

#[apply(main!)]
async fn main() {
    lv_init();
    lv_bevy_ecs::logging::lv_log_init();

    const HOR_RES: u32 = 320;
    const VER_RES: u32 = 240;
    const LINE_HEIGHT: u32 = 16;

    error!("Random error");

    let mut sim_display = SimulatorDisplay::<Rgb565>::new(Size::new(HOR_RES, VER_RES));

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

    lv_tick_set_cb(|| {
        let current_time = SystemTime::now();
        let since_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time should only go forward");
        let ms = since_epoch.as_millis() as u32;
        ms
    });

    let mut world = LvglWorld::default();

    info!("ECS OK");

    {
        let mut button = Button::new();
        let mut label = Label::new();
        label.set_text(c"SPAWN");
        //lv_obj_align(&mut button, LV_ALIGN_CENTER as u8, 10, 10);
        let label_entity = world.spawn(label.into_inner()).id();

        let anim = Animation::new(
            Duration::from_secs(5),
            OpacityLevel::Transparent as i32,
            OpacityLevel::Cover as i32,
            |obj, val| {
                obj.set_style_opa(val as u8, lv_part_t_LV_PART_MAIN as lv_style_selector_t);
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
                    let mut dynamic_button = Button::new();
                    let mut label = Label::new();
                    dynamic_button.set_align(Align::TopRight.into());
                    label.set_text(c"This is dynamic");
                    world
                        .spawn((DynamicButton, dynamic_button.into_inner()))
                        .with_child(label.into_inner());
                }
            }
        });

        let mut button_entity = world.spawn((button.into_inner(), anim));

        button_entity.add_child(label_entity);

        let mut style = Style::default();
        style.set_opa(OpacityLevel::Percent50 as u8);
        style.set_align(Align::TopLeft.into());
        style.set_bg_color(lv_color_make(255, 0, 0));

        button_entity.insert(style);
        //button_entity.remove::<Style>();
        // button_entity.insert(style);

        let mut arc = Arc::new();
        arc.set_align(Align::BottomMid.into());

        world.spawn(arc.into_inner());
    }

    info!("Create OK");

    // let _timer_task = ex.spawn(async {
    //     while !WINDOW_INIT.load(Ordering::Relaxed) {
    //         yield_now().await;
    //     }
    //     loop {
    //         let start = Instant::now();
    //         let next_timer_ms = lv_timer_handler();
    //         Timer::at(start + Duration::from_millis(next_timer_ms.into())).await;
    //     }
    // });

    static WINDOW_INIT: AtomicBool = AtomicBool::new(false);

    let (events_sender, events_receiver) = mpsc::channel();

    let _touch_screen =
        InputDevice::<Pointer>::create(|| get_touch_input(events_receiver.try_iter()));

    // embedded-graphics-simulator uses a blocking fps limiter so async cannot be used
    // a separate thread must be spawned instead
    // https://github.com/embedded-graphics/simulator/issues/70
    let _window_thread = std::thread::spawn(move || {
        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        let mut window = Window::new("Button Example", &output_settings);

        window.update(&sim_display);
        WINDOW_INIT.store(true, Ordering::Relaxed);
        loop {
            // window.events() must be called from this thread on windows
            for event in window.events() {
                events_sender.send(event).unwrap();
            }
            window.update(&sim_display);
        }
    });

    while !WINDOW_INIT.load(Ordering::Relaxed) {
        yield_now().await;
    }

    loop {
        let start = Instant::now();
        let next_timer_ms = lv_timer_handler();
        Timer::at(start + Duration::from_millis(next_timer_ms.into())).await;
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

#[unsafe(no_mangle)]
pub fn get_memory_stats(monitor: &mut lv_bevy_ecs::sys::lv_mem_monitor_t) {
    if let Some(stats) = memory_stats::memory_stats() {
        let memory = stats.physical_mem;
        let virtual_memory = stats.virtual_mem;
        (*monitor).total_size = (virtual_memory) as usize;
        (*monitor).free_size = (virtual_memory - memory) as usize;
        (*monitor).max_used = usize::max((memory) as usize, (*monitor).max_used);
        (*monitor).used_pct = (memory as f64 / virtual_memory as f64 * 100.0) as u8;
    } else {
        error!("Could not retrieve memory stats");
    }
}
