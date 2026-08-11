#![allow(clippy::std_instead_of_core, clippy::std_instead_of_alloc)]

use std::{
    cell::RefCell,
    ffi::CString,
    rc::Rc,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::{Duration, Instant},
};

use lv_bevy_ecs::{
    display::{Display, DrawBuffer},
    error,
    functions::{NextTimerPeriod, lv_init, lv_tick_set_cb, lv_timer_handler},
    input::{BufferStatus, InputDevice, InputEvent, InputState, Pointer},
    support::LvGray8,
    sys::{atkinson, ibmplexsans, lato, montserrat, notosans, sourcesans3, verdana},
    widgets::{Label, LvglWorld},
};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{BinaryColor, Gray8, GrayColor},
    prelude::{Point, Size},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use share_rc::share;

static EXIT_SIGNAL: AtomicBool = AtomicBool::new(false);

fn main() {
    lv_init();
    lv_bevy_ecs::logging::lv_log_init();

    #[cfg(feature = "rust-alloc")]
    lv_bevy_ecs::malloc::set_mem_monitor(get_memory_stats);

    const HOR_RES: usize = 800;
    const VER_RES: usize = 480;
    const LINE_HEIGHT: usize = 16;

    let mut sim_display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(HOR_RES as u32, VER_RES as u32));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("E-Paper simulation", &output_settings);
    window.set_max_fps(0);
    window.update(&sim_display);
    let window_rc = Rc::new(RefCell::new(window));

    error!("Random error");

    let mut display = Display::new(HOR_RES, VER_RES);

    let buffer = DrawBuffer::<LvGray8>::new(HOR_RES, LINE_HEIGHT);

    display.register(
        buffer,
        share!(|refresh| {
            // sim_display.draw_iter(refresh.as_pixels()).unwrap();
            sim_display
                .fill_contiguous(
                    &refresh.rectangle,
                    refresh.colors.iter().map(|px| {
                        //Gray8::from(*px)
                        if Gray8::from(*px).luma() > 96 {
                            BinaryColor::On
                        } else {
                            BinaryColor::Off
                        }
                    }),
                )
                .unwrap();
            if refresh.display.flush_is_last() {
                take!(window_rc.clone()).borrow_mut().update(&sim_display);
            }
        }),
    );

    let _touch_screen = InputDevice::<Pointer>::new(share!(|| get_touch_input(
        take!(window_rc.clone()).borrow().events()
    )));

    drop(window_rc);

    let start = Instant::now();
    lv_tick_set_cb(move || start.elapsed().as_millis() as u32);

    let world = Rc::new(RefCell::new(LvglWorld::default()));

    {
        let world_rc = world.clone();
        let mut world = world_rc.borrow_mut();

        // let mut button = Button::new();
        let mut label = Label::new();
        label.set_pos(10, 10);
        label.set_text(c"This is the default font in a best-case scenario");
        world.spawn(label.into_inner());

        let fonts = unsafe {
            [
                ("Verdana", &verdana),
                ("Atkinson Hyperlegible", &atkinson),
                ("Noto Sans", &notosans),
                ("IBM Plex Sans", &ibmplexsans),
                ("Source Sans 3", &sourcesans3),
                ("Lato", &lato),
                ("Montserrat", &montserrat),
            ]
        };

        for (idx, (name, font)) in fonts.iter().enumerate() {
            let mut label = Label::new();
            label.set_text(Box::leak(
                CString::new(format!(
                    "This font is {} created with the lv_font_conv tool",
                    name
                ))
                .unwrap()
                .into_boxed_c_str(),
            ));
            label.set_pos(10, 40 + idx as i32 * 30);
            label.set_style_text_font(font, 0);
            world.spawn(label.into_inner());
        }
    }

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
                    state: InputState::Pressed,
                    data: point,
                });
                IS_POINTER_DOWN.store(true, Ordering::Relaxed);
            }
            SimulatorEvent::MouseButtonUp { point, .. } => {
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
