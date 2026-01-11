use std::{
    process::exit,
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use lv_bevy_ecs::{
    animation::Animation,
    display::{Display, DrawBuffer},
    error,
    events::Event,
    functions::*,
    info,
    input::{BufferStatus, InputDevice, InputEvent, InputState, Pointer},
    styles::Style,
    support::{Align, OpacityLevel},
    sys::lv_part_t_LV_PART_MAIN,
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
    dynamic_button: Option<Widget>,
    dynamic_button_label: Option<Widget>,
    animation: Option<Animation>,
}

static OBJECTS: LazyLock<Mutex<Objects>> = LazyLock::new(|| Mutex::new(Objects::default()));

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
        trace!("Flushing to display");
        //let _unused = WORLD.lock().unwrap();
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
        diff.as_millis() as u32
    });

    {
        let mut objects = OBJECTS.lock().unwrap();
        let mut button = Button::create_widget();
        let mut label = Label::create_widget();
        lv_label_set_text(&mut label, c"SPAWN");
        lv_obj_set_parent(&mut label, &mut button);

        let mut anim = Animation::new(
            Duration::from_secs(5),
            OpacityLevel::Transparent as i32,
            OpacityLevel::Cover as i32,
            |obj, val| {
                lv_obj_set_style_opa(obj, val as u8, lv_part_t_LV_PART_MAIN);
            },
        );

        anim.set_widget(&mut button);

        lv_obj_add_event_cb(&mut button, Event::Clicked, |_| {
            let mut objects = OBJECTS.lock().unwrap();
            match &objects.dynamic_button {
                Some(_widget) => {
                    objects.dynamic_button = None;
                    objects.dynamic_button_label = None;
                }
                None => {
                    let mut dynamic_button = Button::create_widget();
                    let mut dynamic_label = Label::create_widget();
                    lv_obj_set_align(&mut dynamic_button, Align::TopRight.into());
                    lv_label_set_text(&mut dynamic_label, c"This is dynamic");
                    lv_obj_set_parent(&mut dynamic_label, &mut dynamic_button);
                    objects.dynamic_button = Some(dynamic_button);
                    objects.dynamic_button_label = Some(dynamic_label);
                }
            }
        });

        objects.animation = Some(anim);
        objects.animation.as_mut().unwrap().start();

        let mut style = Style::default();
        lv_style_set_opa(&mut style, OpacityLevel::Percent50 as u8);
        lv_style_set_align(&mut style, Align::TopLeft.into());
        lv_style_set_bg_color(&mut style, lv_color_make(255, 0, 0));

        lv_obj_add_style(&mut button, style, lv_part_t_LV_PART_MAIN);

        button.leak();
        label.leak();

        let mut arc = Arc::create_widget();
        lv_obj_set_align(&mut arc, Align::BottomMid.into());
        arc.leak();
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
