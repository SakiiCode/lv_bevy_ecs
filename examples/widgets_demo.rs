use std::{
    process::exit,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
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
    sys::{LV_DEF_REFR_PERIOD, LV_NO_TIMER_READY, lv_part_t_LV_PART_MAIN},
    widgets::{Arc, Button, Label, LvglWorld},
};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Gray8, Rgb565},
    prelude::{Point, Size},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

#[derive(Component)]
#[component(storage = "SparseSet")]
struct DynamicButton;

fn main() {
    lv_init();
    lv_bevy_ecs::logging::lv_log_init();

    const HOR_RES: u32 = 800;
    const VER_RES: u32 = 480;
    const LINE_HEIGHT: u32 = 16;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Widgets Demo", &output_settings);
    window.set_max_fps(0);

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
        if refresh.display.flush_is_last() {
            window.update(&sim_display);
        }
    });

    info!("Display Driver OK");

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = InputDevice::<Pointer>::create(|| get_touch_input(window.events()));

    info!("Input OK");

    lv_tick_set_cb(|| {
        let current_time = SystemTime::now();
        let since_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time should only go forward");
        let ms = since_epoch.as_millis() as u32;
        ms
    });

    unsafe {
        lv_bevy_ecs::sys::lv_demo_widgets();
    }
    info!("Create OK");

    window.update(&sim_display);

    loop {
        let start = Instant::now();
        let next_timer_ms = lv_timer_handler();
        match next_timer_ms {
            0 => {
                continue;
            }
            LV_NO_TIMER_READY => {
                sleep(Duration::from_millis(LV_DEF_REFR_PERIOD.into()));
            }
            _ => {
                let next_instant = start + Duration::from_millis(next_timer_ms.into());
                sleep(next_instant - Instant::now());
            }
        }
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
