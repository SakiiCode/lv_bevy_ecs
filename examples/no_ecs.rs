#![allow(clippy::std_instead_of_core, clippy::std_instead_of_alloc)]

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::{Duration, Instant},
};

use lightvgl_sys::{
    lv_align_t::{LV_ALIGN_BOTTOM_MID, LV_ALIGN_TOP_LEFT, LV_ALIGN_TOP_RIGHT},
    lv_event_code_t::LV_EVENT_CLICKED,
    lv_indev_state_t::{LV_INDEV_STATE_PRESSED, LV_INDEV_STATE_RELEASED},
    lv_part_t::LV_PART_MAIN,
};
use lv_bevy_ecs::{
    animation::Animation,
    display::{Display, DrawBuffer},
    error,
    functions::*,
    info,
    input::{BufferStatus, InputDevice, InputEvent, Pointer},
    styles::Style,
    support::{LvRgb565, OpacityLevel},
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
use share_rc::share;

#[derive(Default)]
struct Objects {
    dynamic_button: Option<Button<Widget>>,
    dynamic_button_label: Option<Label<Widget>>,
    animation: Option<Animation>,
}

static OBJECTS: LazyLock<Mutex<Objects>> = LazyLock::new(|| Mutex::new(Objects::default()));

static EXIT_SIGNAL: AtomicBool = AtomicBool::new(false);

fn main() {
    lv_init();
    lv_bevy_ecs::logging::lv_log_init();

    #[cfg(feature = "rust-alloc")]
    lv_bevy_ecs::malloc::set_mem_monitor(get_memory_stats);

    const HOR_RES: usize = 320;
    const VER_RES: usize = 240;
    const LINE_HEIGHT: usize = 16;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES as u32, VER_RES as u32));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Button Example", &output_settings);
    window.set_max_fps(0);
    window.update(&sim_display);
    let window_rc = Rc::new(RefCell::new(window));

    info!("SIMULATOR OK");
    error!("Random error");

    let mut display = Display::new(HOR_RES, VER_RES);

    let buffer = DrawBuffer::<LvRgb565>::new(HOR_RES, LINE_HEIGHT);

    info!("Display OK");

    display.register(
        buffer,
        share!(|refresh| {
            //sim_display.draw_iter(refresh.as_pixels()).unwrap();
            trace!("Flushing to display");
            sim_display
                .fill_contiguous(
                    &refresh.rectangle,
                    refresh.colors.iter().map(|px| (*px).into()),
                )
                .unwrap();
            if refresh.display.flush_is_last() {
                take!(window_rc.clone()).borrow_mut().update(&sim_display);
            }
        }),
    );

    info!("Display Driver OK");

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = InputDevice::<Pointer>::new(share!(|| {
        get_touch_input(take!(window_rc.clone()).borrow().events())
    }));

    drop(window_rc);

    info!("Input OK");

    let start = Instant::now();
    lv_tick_set_cb(move || start.elapsed().as_millis() as u32);

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
                obj.set_style_opa(val as u8, LV_PART_MAIN);
            },
        );

        anim.set_widget(&mut button);

        button.add_event_cb(LV_EVENT_CLICKED, |_| {
            let mut objects = OBJECTS.lock().unwrap();
            match &objects.dynamic_button {
                Some(_widget) => {
                    objects.dynamic_button = None;
                    objects.dynamic_button_label = None;
                }
                None => {
                    let mut dynamic_button = Button::new();
                    let mut dynamic_label = Label::new();
                    dynamic_button.set_align(LV_ALIGN_TOP_RIGHT);
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
        style.set_align(LV_ALIGN_TOP_LEFT);
        style.set_bg_color(lv_color_make(255, 0, 0));
        unsafe {
            button.add_style(&mut style, LV_PART_MAIN);
        }

        Button::leak(button);
        Label::leak(label);

        let mut arc = Arc::new();
        arc.set_align(LV_ALIGN_BOTTOM_MID);
        Arc::leak(arc);
    }

    info!("Create OK");

    loop {
        if EXIT_SIGNAL.load(Ordering::Relaxed) {
            break;
        }
        let start = Instant::now();
        let next_timer_period = lv_timer_handler();
        match next_timer_period {
            NextTimerPeriod::Ready => {}
            NextTimerPeriod::AfterMs(next_timer_ms) => {
                let next_instant = start + Duration::from_millis(next_timer_ms.get().into());
                sleep(next_instant - Instant::now());
            }
            NextTimerPeriod::Never => {
                sleep(Duration::from_secs(5));
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
        #[expect(clippy::collapsible_match)]
        match event {
            SimulatorEvent::MouseButtonDown { point, .. } => {
                next_touch_status = Some(InputEvent {
                    status: BufferStatus::Once,
                    state: LV_INDEV_STATE_PRESSED,
                    data: point,
                });
                IS_POINTER_DOWN.store(true, Ordering::Relaxed);
            }
            SimulatorEvent::MouseButtonUp { point, .. } => {
                next_touch_status = Some(InputEvent {
                    status: BufferStatus::Once,
                    state: LV_INDEV_STATE_RELEASED,
                    data: point,
                });
                IS_POINTER_DOWN.store(false, Ordering::Relaxed);
            }
            SimulatorEvent::MouseMove { point } => {
                if IS_POINTER_DOWN.load(Ordering::Relaxed) {
                    next_touch_status = Some(InputEvent {
                        status: BufferStatus::Once,
                        state: LV_INDEV_STATE_PRESSED,
                        data: point,
                    });
                }
            }
            SimulatorEvent::Quit => EXIT_SIGNAL.store(true, Ordering::Relaxed),
            _ => {}
        }
    }

    let mut lock = LATEST_TOUCH_STATUS.lock().unwrap();

    if let Some(latest_touch_status) = next_touch_status {
        *lock = latest_touch_status;
    }
    *lock
}

pub fn get_memory_stats(monitor: &mut lv_bevy_ecs::sys::lv_mem_monitor_t) {
    if let Some(stats) = memory_stats::memory_stats() {
        let memory = stats.physical_mem;
        let virtual_memory = stats.virtual_mem;
        monitor.total_size = virtual_memory;
        monitor.free_size = virtual_memory - memory;
        monitor.max_used = usize::max(memory, monitor.max_used);
        monitor.used_pct = (memory * 100 / virtual_memory) as u8;
    } else {
        error!("Could not retrieve memory stats");
    }
}
