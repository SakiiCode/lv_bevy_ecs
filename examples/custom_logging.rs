use lv_bevy_ecs::{error, functions::lv_log_add, info, trace};

fn main() {
    simple_logger::init().unwrap();
    lv_bevy_ecs::logging::connect();

    info!("This is an info");
    error!("This is an error");
    trace!("This is a trace");

    lv_log_add(
        log::Level::Warn,
        c"custom_logging.rs",
        11,
        c"main",
        c"This warning came from LVGL",
    );

    lv_log_add(
        log::Level::Info,
        c"lv_conf.h",
        395,
        c"main",
        c"You can turn off line numbers with \"LV_LOG_USE_FILE_LINE 0\" in",
    );
}
