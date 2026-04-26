use std::{
    process::exit,
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use lv_bevy_ecs::{
    animation::Animation,
    display::{Display, DrawBuf},
    error,
    events::EventCode,
    functions::*,
    info,
    input::{BufferStatus, Indev, InputEvent, InputState, Pointer},
    styles::Style,
    support::{Align, OpacityLevel},
    sys::{LV_DEF_REFR_PERIOD, lv_part_t_LV_PART_MAIN, lv_style_selector_t},
    trace,
    widgets::{Arc, Button, Label, Widget},
};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::{Point, Size},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

#[derive(Default)]
struct Objects {
    dynamic_button: Option<Button<Widget>>,
    dynamic_button_label: Option<Label<Widget>>,
    animation: Option<Animation>,
}

static OBJECTS: LazyLock<Mutex<Objects>> = LazyLock::new(|| Mutex::new(Objects::default()));

fn main() {
    lv_init();
    lv_bevy_ecs::logging::lv_log_init();

    #[cfg(feature = "rust-alloc")]
    lv_bevy_ecs::malloc::provide_mem_monitor_impl(get_memory_stats);

    const HOR_RES: u32 = 320;
    const VER_RES: u32 = 240;
    const LINE_HEIGHT: u32 = 16;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Button Example", &output_settings);
    window.set_max_fps(0);

    info!("SIMULATOR OK");
    error!("Random error");

    let mut display = Display::new(HOR_RES as i32, VER_RES as i32);

    let buffer = DrawBuf::<{ (HOR_RES * LINE_HEIGHT) as usize }, Rgb565>::new(HOR_RES, LINE_HEIGHT);

    info!("Display OK");

    display.register(buffer, |refresh| {
        //sim_display.draw_iter(refresh.as_pixels()).unwrap();
        trace!("Flushing to display");
        //let _unused = WORLD.lock().unwrap();
        sim_display
            .fill_contiguous(&refresh.rectangle, refresh.colors.iter().cloned())
            .unwrap();
        if refresh.display.flush_is_last() {
            window.update(&sim_display);
        }
        refresh.display.flush_ready();
    });

    info!("Display Driver OK");

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = Indev::<Pointer>::new(|| get_touch_input(window.events()));

    info!("Input OK");

    lv_tick_set_cb(|| {
        let current_time = SystemTime::now();
        let since_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time should only go forward");
        since_epoch.as_millis() as u32
    });

    {
        let mut objects = OBJECTS.lock().unwrap();
        let mut button = Button::new();
        let mut label = Label::new();
        label.set_text(c"SPAWN");
        label.set_parent(&mut button);

        let mut anim = Animation::new(
            Duration::from_secs(5),
            OpacityLevel::Transparent as i32,
            OpacityLevel::Cover as i32,
            |obj, val| {
                obj.set_style_opa(val as u8, lv_part_t_LV_PART_MAIN as lv_style_selector_t);
            },
        );

        anim.set_widget(&mut button);

        button.add_event_cb(EventCode::Clicked, |_| {
            let mut objects = OBJECTS.lock().unwrap();
            match &objects.dynamic_button {
                Some(_widget) => {
                    objects.dynamic_button = None;
                    objects.dynamic_button_label = None;
                }
                None => {
                    let mut dynamic_button = Button::new();
                    let mut dynamic_label = Label::new();
                    dynamic_button.set_align(Align::TopRight.into());
                    dynamic_label.set_text(c"This is dynamic");
                    dynamic_label.set_parent(&mut dynamic_button);
                    objects.dynamic_button = Some(dynamic_button);
                    objects.dynamic_button_label = Some(dynamic_label);
                }
            }
        });

        objects.animation = Some(anim);
        objects.animation.as_mut().unwrap().start();

        let mut style = Box::leak(Box::new(Style::default()));
        style.set_opa(OpacityLevel::Transparent as u8);
        style.set_align(Align::TopLeft.into());
        style.set_bg_color(lv_color_make(255, 0, 0));
        unsafe {
            button.add_style(&mut style, lv_part_t_LV_PART_MAIN as lv_style_selector_t);
        }

        button.leak();
        label.leak();

        let mut arc = Arc::new();
        arc.set_align(Align::BottomMid.into());
        arc.leak();
    }

    info!("Create OK");
    window.update(&sim_display);

    loop {
        let start = Instant::now();
        let next_timer_period = lv_timer_handler();
        match next_timer_period {
            NextTimerPeriod::Ready => {
                continue;
            }
            NextTimerPeriod::AfterMs(next_timer_ms) => {
                let next_instant = start + Duration::from_millis(next_timer_ms.get().into());
                sleep(next_instant - Instant::now());
            }
            NextTimerPeriod::Never => {
                sleep(Duration::from_millis(LV_DEF_REFR_PERIOD.into()));
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
