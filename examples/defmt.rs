use std::sync::atomic::{AtomicUsize, Ordering};

use defmt_stdout as _;
use lv_bevy_ecs::{
    error,
    functions::{lv_init, lv_log_add},
    info, trace,
};

static COUNT: AtomicUsize = AtomicUsize::new(0);
defmt::timestamp!("{=usize}", COUNT.fetch_add(1, Ordering::Relaxed));

fn main() {
    if !cfg!(target_env = "musl") {
        panic!(
            "This example should be run with `cargo run --example defmt --features defmt --target x86_64-unknown-linux-musl`"
        )
    }

    lv_init();
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
