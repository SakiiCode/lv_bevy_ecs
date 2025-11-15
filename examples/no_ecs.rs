use std::{
    process::exit,
    sync::Mutex,
    time::{Duration, Instant},
};

use lazy_static::lazy_static;
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
    spawner_button: Option<Widget>,
    spawner_button_label: Option<Widget>,
    dynamic_button: Option<Widget>,
    dynamic_button_label: Option<Widget>,
    animation: Option<Animation>,
    arc: Option<Widget>,
}

lazy_static! {
    static ref objects_lock: Mutex<Objects> = Mutex::new(Objects::default());
}

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

    info!("ECS OK");

    {
        let mut objects = objects_lock.lock().unwrap();
        //let mut world = WORLD.lock().unwrap();
        let mut button = Button::create_widget();
        let mut label = Label::create_widget();
        lv_label_set_text(&mut label, c"SPAWN");
        lv_obj_set_parent(&mut label, &mut button);
        //lv_obj_align(&mut button, LV_ALIGN_CENTER as u8, 10, 10);
        //let label_entity = world.spawn((Label, label)).id();

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
            //let mut world = WORLD.lock().unwrap();
            let mut objects = objects_lock.lock().unwrap();
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
        objects.spawner_button = Some(button);
        objects.spawner_button_label = Some(label);

        //let mut button_entity = world.spawn((Button, button, anim));

        //button_entity.add_child(label_entity);

        let mut style = Style::default();
        lv_style_set_opa(&mut style, OpacityLevel::Percent50 as u8);
        lv_style_set_align(&mut style, Align::TopLeft.into());
        lv_style_set_bg_color(&mut style, lv_color_make(255, 0, 0));

        lv_obj_add_style(
            objects.spawner_button.as_mut().unwrap(),
            style,
            lv_part_t_LV_PART_MAIN,
        );
        //button_entity.insert(style);
        //button_entity.remove::<Style>();
        // button_entity.insert(style);

        let mut arc = Arc::create_widget();
        lv_obj_set_align(&mut arc, Align::BottomMid.into());
        objects.arc = Some(arc);

        //world.spawn((Arc, arc));
    }

    info!("Create OK");

    let mut is_pointer_down = false;

    let mut prev_time = Instant::now();

    {
        trace!("Window update");
        window.update(&sim_display);
        trace!("Window update done");
    }

    loop {
        let current_time = Instant::now();
        let diff = current_time.duration_since(prev_time);
        prev_time = current_time;

        trace!("Window events");
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
        {
            trace!("LVGL tick update");

            lv_tick_inc(diff);

            trace!("LVGL update");
            lv_timer_handler();
            trace!("Window update");

            window.update(&sim_display);
        }
    }
}
