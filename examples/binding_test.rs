use std::{
    ffi::{CStr, CString},
    process::exit,
    str::FromStr,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use lightvgl_sys::{LV_DEF_REFR_PERIOD, LV_NO_TIMER_READY, lv_display_flush_is_last};
use lv_bevy_ecs::{
    animation::Animation,
    bevy::{component::Component, entity::Entity, hierarchy::Children, query::With, world::World},
    display::{Display, DrawBuffer},
    error,
    events::Event,
    functions::*,
    info,
    input::{BufferStatus, InputDevice, InputEvent, InputState, Pointer},
    styles::Style,
    subjects::Subject,
    support::{LV_SIZE_CONTENT, OpacityLevel},
    sys::{
        LV_ANIM_REPEAT_INFINITE, LV_GRID_CONTENT, LV_GRID_TEMPLATE_LAST, LV_SYMBOL_FILE,
        lv_align_t_LV_ALIGN_BOTTOM_RIGHT, lv_anim_path_ease_out, lv_anim_set_path_cb,
        lv_anim_set_repeat_count, lv_area_t, lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_CHECKED,
        lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_DISABLED,
        lv_chart_axis_t_LV_CHART_AXIS_PRIMARY_X, lv_chart_type_t_LV_CHART_TYPE_BAR,
        lv_chart_type_t_LV_CHART_TYPE_LINE, lv_color_format_t_LV_COLOR_FORMAT_RGB565, lv_color_t,
        lv_draw_buf_align, lv_draw_image_dsc_t, lv_draw_line_dsc_t, lv_event_t,
        lv_flex_flow_t_LV_FLEX_FLOW_COLUMN, lv_font_montserrat_24,
        lv_grid_align_t_LV_GRID_ALIGN_CENTER, lv_grid_align_t_LV_GRID_ALIGN_START,
        lv_grid_align_t_LV_GRID_ALIGN_STRETCH, lv_layer_t, lv_obj_flag_t_LV_OBJ_FLAG_HIDDEN,
        lv_obj_flag_t_LV_OBJ_FLAG_IGNORE_LAYOUT, lv_observer_get_target, lv_observer_t,
        lv_palette_t_LV_PALETTE_BLUE, lv_part_t_LV_PART_ITEMS, lv_state_t_LV_STATE_CHECKED,
        lv_style_selector_t, lv_subject_get_int, lv_subject_t,
    },
    widgets::{
        Button, Buttonmatrix, Canvas, Chart, Dropdown, Image, Label, LvglWorld, Wdg, Widget,
    },
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
struct DynamicLabel;

fn main() {
    lv_init();
    lv_bevy_ecs::logging::lv_log_init();

    #[cfg(feature = "rust-alloc")]
    lv_bevy_ecs::malloc::provide_mem_monitor_impl(get_memory_stats);

    const HOR_RES: u32 = 800;
    const VER_RES: u32 = 480;
    const LINE_HEIGHT: u32 = 10;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Bindings Test Example", &output_settings);
    window.set_max_fps(0);

    let mut display = Display::create(HOR_RES as i32, VER_RES as i32);

    let buffer =
        DrawBuffer::<{ (HOR_RES * LINE_HEIGHT) as usize }, Rgb565>::create(HOR_RES, LINE_HEIGHT);

    let display_ptr = display.raw();

    display.register(buffer, |refresh| {
        //sim_display.draw_iter(refresh.as_pixels()).unwrap();
        sim_display
            .fill_contiguous(&refresh.rectangle, refresh.colors.iter().cloned())
            .unwrap();
        unsafe {
            if lv_display_flush_is_last(display_ptr) {
                window.update(&sim_display);
            }
        }
    });

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = InputDevice::<Pointer>::create(|| get_touch_input(window.events()));

    lv_tick_set_cb(|| {
        let current_time = SystemTime::now();
        let since_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time should only go forward");
        since_epoch.as_millis() as u32
    });

    let mut world = LvglWorld::default();

    create_ui(&mut world);

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

fn create_ui(world: &mut World) {
    let c1: lv_color_t = lv_color_hex(0xff0000);
    let c2: lv_color_t = lv_palette_darken(lv_palette_t_LV_PALETTE_BLUE, 2);
    let c3: lv_color_t = lv_color_mix(c1, c2, OpacityLevel::Percent60 as u8);

    let mut style_big_font = Style::default();
    unsafe {
        style_big_font.set_text_font(&lv_font_montserrat_24);
    }

    let grid_cols = [
        300 as i32,
        lv_grid_fr(3) as i32,
        lv_grid_fr(2) as i32,
        LV_GRID_TEMPLATE_LAST as i32,
    ];
    let grid_rows = [
        100 as i32,
        lv_grid_fr(1) as i32,
        LV_GRID_CONTENT as i32,
        LV_GRID_TEMPLATE_LAST as i32,
    ];

    let mut active_screen = lv_screen_active().unwrap();
    active_screen.set_grid_dsc_array(&grid_cols[0], &grid_rows[0]);

    let mut chart_type_subject = Subject::new_int(0);

    let mut dropdown = Dropdown::new();
    dropdown.set_options(c"Lines\nBars");

    dropdown.set_grid_cell(
        lv_grid_align_t_LV_GRID_ALIGN_CENTER,
        0,
        1,
        lv_grid_align_t_LV_GRID_ALIGN_CENTER,
        0,
        1,
    );

    dropdown.bind_value(chart_type_subject.raw_mut());

    world.spawn(dropdown.into_inner());

    let mut chart = Chart::new();
    chart.set_grid_cell(
        lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
        0,
        1,
        lv_grid_align_t_LV_GRID_ALIGN_CENTER,
        1,
        1,
    );

    let mut series = chart
        .add_series(c3, lv_chart_axis_t_LV_CHART_AXIS_PRIMARY_X)
        .unwrap();
    let mut chart_y_array = [10, 25, 50, 40, 30, 35, 60, 65, 70, 75];
    unsafe {
        chart.set_series_ext_y_array(series.as_mut(), &mut chart_y_array[0]);
    }

    lv_subject_add_observer_obj(&mut chart_type_subject, &mut chart, chart_type_observer_cb);
    lv_subject_set_int(&mut chart_type_subject, 1);

    world.spawn(chart_type_subject);

    world.spawn(chart.into_inner());

    let mut label = Label::new();

    label.set_grid_cell(
        lv_grid_align_t_LV_GRID_ALIGN_START,
        1,
        1,
        lv_grid_align_t_LV_GRID_ALIGN_CENTER,
        0,
        1,
    );

    label.set_style_bg_opa(OpacityLevel::Percent70 as u8, 0);
    label.set_style_bg_color(c1, 0);
    label.set_style_text_color(c2, 0);
    let mut label_entity = world.spawn((DynamicLabel, label.into_inner()));
    label_entity.insert(style_big_font.clone());

    // Converting [&str] to [*const i8] is a little complicated
    // This also leaks memory
    let btnmatrix_options = ["First", "Second", "\n", "Third", ""].map(|s| {
        let cs = CString::from_str(s).unwrap();
        let ptr = cs.as_c_str().as_ptr();
        core::mem::forget(cs);
        ptr
    });

    let btnmatrix_ctrl = Box::new([
        lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_DISABLED,
        2 | lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_CHECKED,
        1,
    ]);

    let mut btnmatrix = Buttonmatrix::new();
    btnmatrix.set_grid_cell(
        lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
        1,
        1,
        lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
        1,
        1,
    );

    btnmatrix.set_map(&btnmatrix_options);

    btnmatrix.set_ctrl_map(&Box::leak(btnmatrix_ctrl)[0]);

    btnmatrix.set_selected_button(1);
    lv_obj_add_event_cb(&mut btnmatrix, Event::ValueChanged, |mut event| {
        buttonmatrix_event_cb(world, &mut event);
    });

    let mut btnmatrix_entity = world.spawn(btnmatrix.into_inner());

    let mut style_big_font_2 =
        Style::new((lv_part_t_LV_PART_ITEMS | lv_state_t_LV_STATE_CHECKED) as lv_style_selector_t);

    unsafe {
        style_big_font_2.set_text_font(&lv_font_montserrat_24);
    }

    btnmatrix_entity.insert(style_big_font_2);

    let mut cont = Widget::new();
    cont.set_grid_cell(
        lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
        2,
        1,
        lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
        0,
        2,
    );
    cont.set_flex_flow(lv_flex_flow_t_LV_FLEX_FLOW_COLUMN);
    let cont_entity = world.spawn(cont);
    let cont_id = cont_entity.id();

    let mut fourth = None;

    for i in 0..10u32 {
        let btn_id = list_button_create(world, cont_id);

        if i == 0 {
            let mut btn_entity = world.get_entity_mut(btn_id).unwrap();

            let mut a = Animation::new(
                Duration::from_millis(300),
                OpacityLevel::Cover as i32,
                OpacityLevel::Percent50 as i32,
                |widget, value| {
                    widget.set_style_opa(value as u8, 0);
                },
            );
            unsafe {
                lv_anim_set_path_cb(a.raw_mut(), Some(lv_anim_path_ease_out));
            }
            btn_entity.insert(a);
        }

        if i == 1 {
            let mut btn_entity = world.get_entity_mut(btn_id).unwrap();

            let mut btn = btn_entity.get_mut::<Widget>().unwrap();
            btn.add_flag(lv_obj_flag_t_LV_OBJ_FLAG_HIDDEN);
        }

        if i == 2 {
            let label_id = {
                let btn_entity = world.get_entity_mut(btn_id).unwrap();
                let children = btn_entity.get::<Children>().unwrap();
                children.first().unwrap().to_owned()
            };
            let mut btn_label_entity = world.get_entity_mut(label_id).unwrap();
            let mut btn_label_widget = btn_label_entity.get_mut::<Widget>().unwrap();
            let btn_label: &mut Label<Wdg> = btn_label_widget.downcast_mut().unwrap();

            btn_label.set_text(c"A multi-line text with a ° symbol");
            btn_label.set_width(lv_pct(100));
        }

        if i == 3 {
            let mut btn_entity = world.get_entity_mut(btn_id).unwrap();
            fourth = Some(btn_id);

            let mut a = Animation::new(
                Duration::from_millis(300),
                OpacityLevel::Cover as i32,
                OpacityLevel::Percent50 as i32,
                opa_anim_cb,
            );
            unsafe {
                lv_anim_set_repeat_count(a.raw_mut(), LV_ANIM_REPEAT_INFINITE);
            }
            btn_entity.insert(a);
        }
    }

    sleep(Duration::from_millis(300));
    if let Some(fourth) = fourth {
        world.despawn(fourth);
    }

    let mut canvas_buf = [0u8; 400 * 100 * 4];

    let mut canvas = Canvas::new();
    canvas.set_grid_cell(
        lv_grid_align_t_LV_GRID_ALIGN_START,
        0,
        2,
        lv_grid_align_t_LV_GRID_ALIGN_START,
        2,
        1,
    );

    unsafe {
        let buf = lv_draw_buf_align(
            canvas_buf.as_mut_ptr().cast(),
            lv_color_format_t_LV_COLOR_FORMAT_RGB565,
        );
        canvas.set_buffer(
            buf.as_mut().unwrap(),
            400,
            100,
            lv_color_format_t_LV_COLOR_FORMAT_RGB565,
        );
    }

    canvas.fill_bg(c2, OpacityLevel::Cover as u8);

    draw_to_canvas(&mut canvas);

    world.spawn(canvas.into_inner());
    let test_img_lvgl_logo_png_path = c"A:examples/assets/test_img_lvgl_logo.png";
    let test_img_lvgl_logo_png = unsafe { test_img_lvgl_logo_png_path.as_ptr().as_ref().unwrap() };

    let test_img_lvgl_logo_jpg_path = c"A:examples/assets/test_img_lvgl_logo.jpg";
    let test_img_lvgl_logo_jpg = unsafe { test_img_lvgl_logo_jpg_path.as_ptr().as_ref().unwrap() };

    let mut img = Image::new();
    img.set_src(test_img_lvgl_logo_jpg);

    img.align(lv_align_t_LV_ALIGN_BOTTOM_RIGHT, -20, -20);
    img.add_flag(lv_obj_flag_t_LV_OBJ_FLAG_IGNORE_LAYOUT);
    world.spawn(img.into_inner());

    let mut img = Image::new();
    img.set_src(test_img_lvgl_logo_png);

    img.set_pos(500, 420);
    img.add_flag(lv_obj_flag_t_LV_OBJ_FLAG_IGNORE_LAYOUT);
    img.set_rotation(200);
    img.set_scale_x(400);
    world.spawn(img.into_inner());
}

fn opa_anim_cb(widget: &mut Wdg, value: i32) {
    widget.set_style_opa(value as u8, 0);
}

fn chart_type_observer_cb(observer: *mut lv_observer_t, subject: *mut lv_subject_t) {
    info!("chart_type_observer_cb");
    unsafe {
        let v = lv_subject_get_int(subject);
        let mut chart_wdg = Wdg::from_ptr(lv_observer_get_target(observer).cast());
        let chart: &mut Chart<Wdg> = chart_wdg.downcast_mut().unwrap();
        let chart_type = if v == 0 {
            lv_chart_type_t_LV_CHART_TYPE_LINE
        } else {
            lv_chart_type_t_LV_CHART_TYPE_BAR
        };
        chart.set_type(chart_type);
    }
}

fn buttonmatrix_event_cb(world: &mut World, e: &mut lv_event_t) {
    // lv_event_get_user_data must not be used!
    // user data is reserved for the callback function
    let btnmatrix = Wdg::from_ptr(lv_event_get_target(e).cast_mut().cast());
    let buttonmatrix: &Buttonmatrix<Wdg> = btnmatrix.downcast().unwrap();

    let idx = buttonmatrix.get_selected_button();
    let text = buttonmatrix.get_button_text(idx);
    let mut label_widget = world
        .query_filtered::<&mut Widget, With<DynamicLabel>>()
        .single_mut(world)
        .unwrap();
    let label: &mut Label<Wdg> = label_widget.downcast_mut().unwrap();

    label.set_text(text);
}

fn list_button_create(world: &mut World, parent: Entity) -> Entity {
    let mut btn = Button::new();
    btn.set_size(lv_pct(100), LV_SIZE_CONTENT as i32);

    let btn_id = world.spawn(btn.into_inner()).id();
    let mut parent = world.entity_mut(parent);
    parent.add_child(btn_id);

    let btn = world.get::<Widget>(btn_id).unwrap();
    let idx = btn.get_index();

    info!("Spawning button {}", idx);

    let mut label = Label::new();
    let file_icon_str = CStr::from_bytes_with_nul(LV_SYMBOL_FILE).unwrap();
    let file_icon = file_icon_str.to_string_lossy();

    label.set_text(
        CString::new(format!("{} Item {}", file_icon, idx))
            .unwrap()
            .as_c_str(),
    );

    let label_id = world.spawn(label.into_inner()).id();
    world.get_entity_mut(btn_id).unwrap().add_child(label_id);

    btn_id
}

fn draw_to_canvas(canvas: &mut Canvas<Wdg>) {
    let mut layer = unsafe {
        let mut layer = std::mem::MaybeUninit::<lv_layer_t>::uninit();
        lightvgl_sys::lv_canvas_init_layer(canvas.raw_mut(), layer.as_mut_ptr());

        layer.assume_init()
    };

    /*Use draw descriptors*/
    let test_img_lvgl_logo_png_path = c"A:examples/assets/test_img_lvgl_logo.png".as_ptr();
    let test_img_lvgl_logo_png = test_img_lvgl_logo_png_path.cast();
    let mut image_draw_dsc = unsafe {
        let mut image_draw_dsc = std::mem::MaybeUninit::<lv_draw_image_dsc_t>::uninit();
        lightvgl_sys::lv_draw_image_dsc_init(image_draw_dsc.as_mut_ptr());
        image_draw_dsc.assume_init()
    };
    image_draw_dsc.src = test_img_lvgl_logo_png;

    const WIDTH: i32 = 105;
    const HEIGHT: i32 = 40;

    let mut coords: lv_area_t = lv_area_t {
        x1: 10,
        y1: 10,
        x2: 10 + WIDTH - 1,
        y2: 10 + HEIGHT - 1,
    };
    unsafe {
        lightvgl_sys::lv_draw_image(&mut layer, &image_draw_dsc, &coords);

        /*Reuse the draw descriptor*/
        lightvgl_sys::lv_area_move(&mut coords, 40, 40);
        image_draw_dsc.opa = OpacityLevel::Percent50 as u8;
        lightvgl_sys::lv_draw_image(&mut layer, &image_draw_dsc, &coords);

        let mut line_draw_dsc = std::mem::MaybeUninit::<lv_draw_line_dsc_t>::uninit();
        lightvgl_sys::lv_draw_line_dsc_init(line_draw_dsc.as_mut_ptr());
        let mut line_draw_dsc = line_draw_dsc.assume_init();
        line_draw_dsc.color = lv_color_hex3(0xCA8);
        line_draw_dsc.width = 8;
        line_draw_dsc.set_round_start(1);
        line_draw_dsc.set_round_end(1);
        lightvgl_sys::lv_point_precise_set(&mut line_draw_dsc.p1, 150, 30);
        lightvgl_sys::lv_point_precise_set(&mut line_draw_dsc.p2, 350, 55);
        lightvgl_sys::lv_draw_line(&mut layer, &line_draw_dsc);

        lightvgl_sys::lv_canvas_finish_layer(canvas.raw_mut(), &mut layer);

        let c = lv_color_make(255, 0, 0);
        for i in 0..50 {
            lightvgl_sys::lv_canvas_set_px(
                canvas.raw_mut(),
                100 + i * 2,
                10,
                c,
                OpacityLevel::Cover as u8,
            );
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
