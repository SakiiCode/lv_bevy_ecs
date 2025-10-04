use std::{
    ffi::{CStr, CString, c_void},
    process::exit,
    thread::sleep,
    time::Duration,
};

use bevy_ecs::{hierarchy::Children, query::With};
use lv_bevy_ecs::{
    LvglSchedule, LvglWorld,
    animation::Animation,
    display::{Display, DrawBuffer},
    events::{Event, lv_event_get_target, lv_obj_add_event_cb},
    functions::{
        lv_buttonmatrix_set_ctrl_map, lv_buttonmatrix_set_selected_button, lv_canvas_fill_bg,
        lv_canvas_set_buffer, lv_chart_set_ext_y_array, lv_dropdown_set_options,
        lv_image_set_rotation, lv_image_set_scale_x, lv_image_set_src, lv_label_set_text,
        lv_obj_add_flag, lv_obj_align, lv_obj_get_index, lv_obj_set_flex_flow,
        lv_obj_set_grid_cell, lv_obj_set_pos, lv_obj_set_style_bg_color, lv_obj_set_style_bg_opa,
        lv_obj_set_style_opa, lv_obj_set_style_text_color, lv_obj_set_width,
        lv_style_set_text_font, lv_timer_handler,
    },
    input::{InputDevice, PointerInputData},
    subjects::{Subject, lv_subject_add_observer_obj, lv_subject_set_int},
    support::{Color, LvError, lv_pct},
    widgets::{Buttonmatrix, Canvas, Chart, Dropdown, Image, Widget},
};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::{Point, Size},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lv_bevy_ecs::styles::Style;
use lv_bevy_ecs::widgets::{Button, Label};

use lightvgl_sys::{
    LV_GRID_CONTENT, LV_GRID_TEMPLATE_LAST, LV_OPA_50, LV_OPA_60, LV_OPA_70, LV_OPA_COVER,
    LV_PART_ITEMS, LV_STATE_CHECKED, LV_SYMBOL_FILE, lv_align_t_LV_ALIGN_BOTTOM_RIGHT, lv_area_t,
    lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_CHECKED,
    lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_DISABLED, lv_buttonmatrix_get_button_text,
    lv_buttonmatrix_get_selected_button, lv_chart_add_series,
    lv_chart_axis_t_LV_CHART_AXIS_PRIMARY_X, lv_chart_set_type, lv_chart_type_t_LV_CHART_TYPE_BAR,
    lv_chart_type_t_LV_CHART_TYPE_LINE, lv_color_hex3, lv_color_mix, lv_color_t, lv_draw_buf_align,
    lv_draw_image_dsc_t, lv_draw_line_dsc_t, lv_dropdown_bind_value, lv_event_t,
    lv_flex_flow_t_LV_FLEX_FLOW_COLUMN, lv_font_montserrat_24,
    lv_grid_align_t_LV_GRID_ALIGN_CENTER, lv_grid_align_t_LV_GRID_ALIGN_START,
    lv_grid_align_t_LV_GRID_ALIGN_STRETCH, lv_layer_t, lv_obj_create,
    lv_obj_flag_t_LV_OBJ_FLAG_HIDDEN, lv_obj_flag_t_LV_OBJ_FLAG_IGNORE_LAYOUT,
    lv_observer_get_target, lv_observer_t, lv_palette_darken, lv_palette_t_LV_PALETTE_BLUE,
    lv_screen_active, lv_subject_get_int, lv_subject_t,
};
use lv_bevy_ecs::prelude::{
    component::Component, entity::Entity, lv_color_format_t_LV_COLOR_FORMAT_RGB565,
    lv_indev_type_t_LV_INDEV_TYPE_POINTER, world::World,
};

macro_rules! cstr {
    ($txt:literal) => {{
        const STR: &[u8] = concat!($txt, "\0").as_bytes();
        unsafe { CStr::from_bytes_with_nul_unchecked(STR) }
    }};
}

macro_rules! lv_grid_fr {
    ($x:literal) => {
        lightvgl_sys::LV_COORD_MAX - 100 + $x
    };
}

#[derive(Component)]
struct DynamicLabel;

fn main() -> Result<(), LvError> {
    const HOR_RES: u32 = 800;
    const VER_RES: u32 = 480;
    const LINE_HEIGHT: u32 = 10;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Bindings Test Example", &output_settings);
    println!("SIMULATOR OK");

    let mut display = Display::create(HOR_RES as i32, VER_RES as i32);

    let buffer =
        DrawBuffer::<{ (HOR_RES * LINE_HEIGHT) as usize }, Rgb565>::create(HOR_RES, LINE_HEIGHT);

    println!("Display OK");

    display.register(buffer, |refresh| {
        //sim_display.draw_iter(refresh.as_pixels()).unwrap();
        sim_display
            .fill_contiguous(
                &refresh.rectangle,
                refresh.colors.iter().cloned().map(|c| c.into()),
            )
            .unwrap();
    });

    println!("Display Driver OK");

    // Define the initial state of your input
    let mut latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = InputDevice::create(lv_indev_type_t_LV_INDEV_TYPE_POINTER, || {
        latest_touch_status
    });

    println!("Input OK");

    let mut world = LvglWorld::new();

    println!("ECS OK");

    let btnmatrix_options = [
        cstr!("First").as_ptr(),
        cstr!("Second").as_ptr(),
        cstr!("\n").as_ptr(),
        cstr!("Third").as_ptr(),
        cstr!("").as_ptr(),
    ];

    let btnmatrix_ctrl = [
        lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_DISABLED,
        2 | lv_buttonmatrix_ctrl_t_LV_BUTTONMATRIX_CTRL_CHECKED,
        1,
    ];

    {
        let c1: lv_color_t = Color::from_rgb(255, 0, 0).into();
        let c2: lv_color_t = unsafe { lv_palette_darken(lv_palette_t_LV_PALETTE_BLUE, 2) };
        let c3: lv_color_t = unsafe { lv_color_mix(c1, c2, LV_OPA_60 as u8) };

        let mut style_big_font = Style::default();
        unsafe {
            lv_style_set_text_font(&mut style_big_font, &lv_font_montserrat_24);
        }

        let mut grid_cols = [
            300 as i32,
            lv_grid_fr!(3) as i32,
            lv_grid_fr!(2) as i32,
            LV_GRID_TEMPLATE_LAST as i32,
        ];
        let mut grid_rows = [
            100 as i32,
            lv_grid_fr!(1) as i32,
            LV_GRID_CONTENT as i32,
            LV_GRID_TEMPLATE_LAST as i32,
        ];

        unsafe {
            lightvgl_sys::lv_obj_set_grid_dsc_array(
                lv_screen_active(),
                grid_cols.as_mut_ptr(),
                grid_rows.as_mut_ptr(),
            );
        }

        let mut chart_type_subject = Subject::new_int(0);

        let mut dropdown = Dropdown::create_widget()?;
        lv_dropdown_set_options(&mut dropdown, &cstr!("Lines\nBars"));

        lv_obj_set_grid_cell(
            &mut dropdown,
            lv_grid_align_t_LV_GRID_ALIGN_CENTER,
            0,
            1,
            lv_grid_align_t_LV_GRID_ALIGN_CENTER,
            0,
            1,
        );

        unsafe {
            lv_dropdown_bind_value(dropdown.raw(), chart_type_subject.raw());
        }

        world.spawn((Dropdown, dropdown));

        let mut chart = Chart::create_widget()?;
        lv_obj_set_grid_cell(
            &mut chart,
            lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
            0,
            1,
            lv_grid_align_t_LV_GRID_ALIGN_CENTER,
            1,
            1,
        );

        unsafe {
            let series =
                lv_chart_add_series(chart.raw(), c3, lv_chart_axis_t_LV_CHART_AXIS_PRIMARY_X);
            let mut chart_y_array = [10, 25, 50, 40, 30, 35, 60, 65, 70, 75];

            lv_chart_set_ext_y_array(&mut chart, series.as_mut().unwrap(), &mut chart_y_array[0]);
        }

        lv_subject_add_observer_obj(&mut chart_type_subject, &mut chart, chart_type_observer_cb);
        lv_subject_set_int(&mut chart_type_subject, 1);

        world.spawn(chart_type_subject);

        world.spawn((Chart, chart));

        let mut label = Label::create_widget()?;

        lv_obj_set_grid_cell(
            &mut label,
            lv_grid_align_t_LV_GRID_ALIGN_START,
            1,
            1,
            lv_grid_align_t_LV_GRID_ALIGN_CENTER,
            0,
            1,
        );

        lv_obj_set_style_bg_opa(&mut label, LV_OPA_70 as u8, 0);
        lv_obj_set_style_bg_color(&mut label, c1, 0);
        lv_obj_set_style_text_color(&mut label, c2, 0);
        let mut label_entity = world.spawn((DynamicLabel, Label, label));
        label_entity.insert(style_big_font.clone());

        let mut btnmatrix = Buttonmatrix::create_widget()?;
        unsafe {
            lv_obj_set_grid_cell(
                &mut btnmatrix,
                lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
                1,
                1,
                lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
                1,
                1,
            );

            lightvgl_sys::lv_buttonmatrix_set_map(btnmatrix.raw(), &btnmatrix_options[0]);
            lv_buttonmatrix_set_ctrl_map(&mut btnmatrix, &btnmatrix_ctrl[0]);

            lv_buttonmatrix_set_selected_button(&mut btnmatrix, 1);
            lv_obj_add_event_cb(&btnmatrix, Event::ValueChanged, |mut event| {
                buttonmatrix_event_cb(&mut world, &mut event);
            });
        }
        let mut btnmatrix_entity = world.spawn((Buttonmatrix, btnmatrix));
        let mut style_big_font_2 = Style::new(LV_PART_ITEMS | LV_STATE_CHECKED);
        unsafe {
            lv_style_set_text_font(&mut style_big_font_2, &lv_font_montserrat_24);
        }

        btnmatrix_entity.insert(style_big_font_2);

        let mut cont = unsafe { Widget::from_raw(lv_obj_create(lv_screen_active())).unwrap() };
        lv_obj_set_grid_cell(
            &mut cont,
            lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
            2,
            1,
            lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
            0,
            2,
        );
        lv_obj_set_flex_flow(&mut cont, lv_flex_flow_t_LV_FLEX_FLOW_COLUMN);
        let cont_entity = world.spawn(cont);
        let cont_id = cont_entity.id();

        let mut fourth = None;

        for i in 0..10u32 {
            let btn_id = list_button_create(&mut world, cont_id)?;

            if i == 0 {
                let mut btn_entity = world.get_entity_mut(btn_id).unwrap();

                let a = Animation::new(
                    Duration::from_millis(300),
                    LV_OPA_COVER as i32,
                    LV_OPA_50 as i32,
                    |widget, value| {
                        lv_obj_set_style_opa(widget, value as u8, 0);
                    },
                );
                btn_entity.insert(a);
            }

            if i == 1 {
                let mut btn_entity = world.get_entity_mut(btn_id).unwrap();

                let mut btn = btn_entity.get_mut::<Widget>().unwrap();
                lv_obj_add_flag(&mut btn, lv_obj_flag_t_LV_OBJ_FLAG_HIDDEN);
            }

            if i == 2 {
                let label_id;
                {
                    let btn_entity = world.get_entity_mut(btn_id).unwrap();
                    let children = btn_entity.get::<Children>().unwrap();
                    label_id = children.first().unwrap().to_owned();
                }
                let mut btn_label_entity = world.get_entity_mut(label_id).unwrap();
                let mut btn_label = btn_label_entity.get_mut::<Widget>().unwrap();

                lv_label_set_text(&mut btn_label, cstr!("A multi-line text with a ° symbol"));

                lv_obj_set_width(&mut btn_label, lv_pct(100));
            }

            if i == 3 {
                let mut btn_entity = world.get_entity_mut(btn_id).unwrap();

                fourth = Some(btn_id);
                let a = Animation::new(
                    Duration::from_millis(300),
                    LV_OPA_COVER as i32,
                    LV_OPA_50 as i32,
                    |widget, value| {
                        lv_obj_set_style_opa(widget, value as u8, 0);
                    },
                );
                btn_entity.insert(a);
            }
        }

        sleep(Duration::from_millis(300));
        if let Some(fourth) = fourth {
            world.despawn(fourth);
        }

        let mut canvas_buf = [0u8; 400 * 100 * 2];

        let mut canvas = Canvas::create_widget()?;
        lv_obj_set_grid_cell(
            &mut canvas,
            lv_grid_align_t_LV_GRID_ALIGN_START,
            0,
            2,
            lv_grid_align_t_LV_GRID_ALIGN_START,
            2,
            1,
        );

        unsafe {
            lv_canvas_set_buffer(
                &mut canvas,
                lv_draw_buf_align(
                    (canvas_buf.as_mut_ptr() as *mut c_void).as_mut().unwrap(),
                    lv_color_format_t_LV_COLOR_FORMAT_RGB565,
                )
                .as_mut()
                .unwrap(),
                400,
                100,
                lv_color_format_t_LV_COLOR_FORMAT_RGB565,
            );
        }

        lv_canvas_fill_bg(&mut canvas, c2, LV_OPA_COVER as u8);

        draw_to_canvas(&mut canvas);

        world.spawn((Canvas, canvas));

        let test_img_lvgl_logo_png_path =
            cstr!("A:examples/assets/test_img_lvgl_logo.png").as_ptr();
        let test_img_lvgl_logo_png = unsafe {
            (test_img_lvgl_logo_png_path as *mut c_void)
                .as_mut()
                .unwrap()
        };

        let test_img_lvgl_logo_jpg_path =
            cstr!("A:examples/assets/test_img_lvgl_logo.jpg").as_ptr();
        let test_img_lvgl_logo_jpg = unsafe {
            (test_img_lvgl_logo_jpg_path as *mut c_void)
                .as_mut()
                .unwrap()
        };

        let mut img = Image::create_widget()?;
        lv_image_set_src(&mut img, test_img_lvgl_logo_jpg);
        lv_obj_align(&mut img, lv_align_t_LV_ALIGN_BOTTOM_RIGHT, -20, -20);
        lv_obj_add_flag(&mut img, lv_obj_flag_t_LV_OBJ_FLAG_IGNORE_LAYOUT);
        world.spawn((Image, img));

        let mut img = Image::create_widget()?;
        lv_image_set_src(&mut img, test_img_lvgl_logo_png);
        lv_obj_set_pos(&mut img, 500, 420);
        lv_obj_add_flag(&mut img, lv_obj_flag_t_LV_OBJ_FLAG_IGNORE_LAYOUT);
        lv_image_set_rotation(&mut img, 200);
        lv_image_set_scale_x(&mut img, 400);
        world.spawn((Image, img));
    }

    println!("Create OK");
    // Create a new Schedule, which defines an execution strategy for Systems
    let mut schedule = LvglSchedule::new();

    let mut is_pointer_down = false;

    loop {
        window.update(&sim_display);
        let events = window.events().peekable();

        for event in events {
            #[allow(unused_assignments)]
            match event {
                SimulatorEvent::MouseButtonDown {
                    mouse_btn: _,
                    point,
                } => {
                    println!("Clicked on: {:?}", point);
                    latest_touch_status = PointerInputData::Touch(point).pressed().once();
                    is_pointer_down = true;
                }
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => {
                    latest_touch_status = PointerInputData::Touch(point).released().once();
                    is_pointer_down = false;
                }
                SimulatorEvent::MouseMove { point } => {
                    if is_pointer_down {
                        latest_touch_status = PointerInputData::Touch(point).pressed().once();
                    }
                }
                SimulatorEvent::Quit => exit(0),
                _ => {}
            }
        }

        // Run the schedule once. If your app has a "loop", you would run this once per loop
        schedule.run(&mut world);

        lv_timer_handler();
    }
}

fn chart_type_observer_cb(observer: *mut lv_observer_t, subject: *mut lv_subject_t) {
    println!("chart_type_observer_cb");
    unsafe {
        let v = lv_subject_get_int(subject);
        let chart = lv_observer_get_target(observer);
        lv_chart_set_type(
            chart as *mut lightvgl_sys::lv_obj_t,
            if v == 0 {
                lv_chart_type_t_LV_CHART_TYPE_LINE
            } else {
                lv_chart_type_t_LV_CHART_TYPE_BAR
            },
        );
    }
}

fn buttonmatrix_event_cb(world: &mut World, e: &mut lv_event_t) {
    unsafe {
        let buttonmatrix = lv_event_get_target(e) as *const lightvgl_sys::lv_obj_t;

        let idx = lv_buttonmatrix_get_selected_button(buttonmatrix);
        let text = lv_buttonmatrix_get_button_text(buttonmatrix, idx);
        let text_owned = CStr::from_ptr(text).to_string_lossy().into_owned();
        let text_cstring = CString::new(text_owned).unwrap();
        //lightvgl_sys::lv_label_set_text(label, text_cstring.as_ptr());
        for mut label in world
            .query_filtered::<&mut Widget, With<DynamicLabel>>()
            .iter_mut(world)
        {
            lv_label_set_text(&mut label, text_cstring.as_c_str());
        }

        //std::mem::forget(text_cstring);
    }
}

fn list_button_create(world: &mut World, parent: Entity) -> Result<Entity, LvError> {
    let mut btn = Button::create_widget()?;
    //lv_obj_set_size(&mut btn, lv_pct(100), LV_SIZE_CONTENT as i32);
    lv_obj_set_width(&mut btn, lv_pct(100));

    let btn_id = world.spawn((Button, btn)).id();
    let mut parent = world.get_entity_mut(parent).unwrap();
    parent.add_child(btn_id);

    let idx = lv_obj_get_index(world.get_entity(btn_id).unwrap().get::<Widget>().unwrap());
    println!("Spawning button {}", idx);

    let mut label = Label::create_widget()?;
    let file_icon_str = CStr::from_bytes_with_nul(LV_SYMBOL_FILE).unwrap();
    let file_icon = file_icon_str.to_string_lossy();

    lv_label_set_text(
        &mut label,
        CString::new(format!("{} Item {}", file_icon, idx))
            .unwrap()
            .as_c_str(),
    );

    let label_id = world.spawn((Label, label)).id();
    world.get_entity_mut(btn_id).unwrap().add_child(label_id);

    Ok(btn_id)
}

fn draw_to_canvas(canvas: &mut Widget) {
    let mut layer = unsafe {
        let mut layer = std::mem::MaybeUninit::<lv_layer_t>::uninit();
        lightvgl_sys::lv_canvas_init_layer(canvas.raw(), layer.as_mut_ptr());

        layer.assume_init()
    };

    /*Use draw descriptors*/
    let test_img_lvgl_logo_png_path = cstr!("A:examples/assets/test_img_lvgl_logo.png").as_ptr();
    let test_img_lvgl_logo_png = unsafe {
        (test_img_lvgl_logo_png_path as *mut c_void)
            .as_mut()
            .unwrap()
    };
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
        image_draw_dsc.opa = LV_OPA_50 as u8;
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

        lightvgl_sys::lv_canvas_finish_layer(canvas.raw(), &mut layer);

        let c = Color::from_rgb(255, 0, 0);
        for i in 0..50 {
            lightvgl_sys::lv_canvas_set_px(
                canvas.raw(),
                100 + i * 2,
                10,
                c.into(),
                LV_OPA_COVER as u8,
            );
        }
    }
}
