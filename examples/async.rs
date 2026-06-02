#![allow(clippy::std_instead_of_core, clippy::std_instead_of_alloc)]

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
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
    events::EventCode,
    functions::*,
    info,
    input::{BufferStatus, InputDevice, InputEvent, InputState, Pointer},
    styles::Style,
    support::{Align, OpacityLevel},
    sys::{lv_part_t_LV_PART_MAIN, lv_style_selector_t},
    widgets::{Arc, Button, Label, UnsafeLvglWorld},
};
use macro_rules_attribute::apply;
use smol::{Timer, future::yield_now};
use smol_macros::main;

#[derive(Component)]
struct DynamicButton;

static WORLD: Mutex<UnsafeLvglWorld> = Mutex::new(UnsafeLvglWorld::new());

static EXIT_SIGNAL: AtomicBool = AtomicBool::new(false);

#[apply(main!)]
async fn main() {
    lv_init();
    lv_bevy_ecs::logging::lv_log_init();

    const HOR_RES: usize = 320;
    const VER_RES: usize = 240;
    const LINE_HEIGHT: usize = 16;

    let mut sim_display =
        SimulatorDisplay::<Rgb565>::new(Size::new(HOR_RES as u32, VER_RES as u32));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Async Button Example", &output_settings);
    window.set_max_fps(0);

    window.update(&sim_display);

    let window_rc = Rc::new(RefCell::new(window));

    info!("Simulator OK");

    let mut display = Display::new(HOR_RES, VER_RES);

    let buffer = DrawBuffer::<{ HOR_RES * LINE_HEIGHT }, Rgb565>::new(HOR_RES, LINE_HEIGHT);

    info!("Display OK");

    let window = window_rc.clone();
    display.register(buffer, move |refresh| {
        //sim_display.draw_iter(refresh.as_pixels()).unwrap();
        sim_display
            .fill_contiguous(&refresh.rectangle, refresh.colors.iter().copied())
            .unwrap();
        if refresh.display.flush_is_last() {
            window.borrow_mut().update(&sim_display);
        }
    });

    info!("Display Driver OK");

    let window = window_rc;
    let _touch_screen =
        InputDevice::<Pointer>::new(move || get_touch_input(window.borrow().events()));

    let start = Instant::now();
    lv_tick_set_cb(move || start.elapsed().as_millis() as u32);

    info!("ECS OK");

    {
        let mut world = WORLD.lock().unwrap();
        world.init();

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
                #[expect(clippy::cast_sign_loss)]
                obj.set_style_opa(val as u8, lv_part_t_LV_PART_MAIN as lv_style_selector_t);
            },
        );

        button.add_event_cb(EventCode::Clicked, |_| {
            let mut world = WORLD.lock().unwrap();
            if let Ok(entity) = world
                .query_filtered::<Entity, With<DynamicButton>>()
                .single(&world)
            {
                world.despawn(entity);
            } else {
                let mut dynamic_button = Button::new();
                let mut label = Label::new();
                dynamic_button.set_align(Align::TopRight.into());
                label.set_text(c"This is dynamic");
                world
                    .spawn((DynamicButton, dynamic_button.into_inner()))
                    .with_child(label.into_inner());
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

    loop {
        if EXIT_SIGNAL.load(Ordering::Relaxed) {
            break;
        }
        let start = Instant::now();
        let next_timer_period = lv_timer_handler();
        match next_timer_period {
            NextTimerPeriod::Ready => {
                yield_now().await;
            }
            NextTimerPeriod::Never => {
                Timer::after(Duration::from_secs(5)).await;
            }
            NextTimerPeriod::AfterMs(next_timer_ms) => {
                Timer::at(start + Duration::from_millis(next_timer_ms.get().into())).await;
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
