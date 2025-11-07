use log::Level;

use std::ffi::{CStr, CString};

macro_rules! cstr {
    ($txt:expr) => {
        CString::new($txt).unwrap().as_c_str()
    };
}

#[macro_export]
macro_rules! func {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name[..name.len() - 3].split("::").last().unwrap()
    }};
}

/// Forward LVGL logging to the `log` crate
///
/// Must not be used together with lv_log_init();
pub fn connect() {
    unsafe {
        lightvgl_sys::lv_log_register_print_cb(Some(lvgl_log));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn lvgl_log(level: lightvgl_sys::lv_log_level_t, buf: *const core::ffi::c_char) {
    let message = unsafe { CStr::from_ptr(buf).to_owned() };
    let message = message.to_string_lossy();
    let message = message.trim();
    let parts = message.split(':').collect::<Vec<&str>>();
    let target = parts[0].split(" ").last().unwrap();
    let message = parts[1..].join(":");
    match level as u32 {
        lightvgl_sys::LV_LOG_LEVEL_TRACE => {
            log::trace!(target:target, "{}", message.trim());
        }
        lightvgl_sys::LV_LOG_LEVEL_INFO => {
            log::info!(target:target, "{}", message.trim());
        }
        lightvgl_sys::LV_LOG_LEVEL_WARN => {
            log::warn!(target:target, "{}", message.trim());
        }
        lightvgl_sys::LV_LOG_LEVEL_ERROR => {
            log::error!(target:target, "{}", message.trim());
        }
        _ => {
            log::error!("Invalid log level: {level}");
        }
    }
}

/// Use LVGL as the backend for the `log` crate.
///
/// Must not be used together with `lv_bevy_ecs::logging::connect()`
pub fn lv_log_init() {
    match log::set_logger(&LvglLogger) {
        Ok(_) => log::set_max_level(log::LevelFilter::Trace),
        Err(err) => println!("Could not initialize logging: {}", err.to_string()),
    }
}

pub fn as_lv_log_level(level: Level) -> lightvgl_sys::lv_log_level_t {
    (match level {
        Level::Trace => lightvgl_sys::LV_LOG_LEVEL_TRACE,
        Level::Debug => lightvgl_sys::LV_LOG_LEVEL_TRACE,
        Level::Info => lightvgl_sys::LV_LOG_LEVEL_INFO,
        Level::Warn => lightvgl_sys::LV_LOG_LEVEL_WARN,
        Level::Error => lightvgl_sys::LV_LOG_LEVEL_ERROR,
    }) as lightvgl_sys::lv_log_level_t
}

pub(crate) fn lv_log_add(
    level: Level,
    file: &core::ffi::CStr,
    line: u32,
    func: &core::ffi::CStr,
    message: &core::ffi::CStr,
) {
    unsafe {
        lightvgl_sys::lv_log_add(
            as_lv_log_level(level),
            file.as_ptr(),
            line as i32,
            func.as_ptr(),
            message.as_ptr(),
        );
    }
}

pub struct LvglLogger;

impl log::Log for LvglLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        lv_log_add(
            record.level(),
            cstr!(record.file().unwrap_or_default()),
            record.line().unwrap_or_default(),
            cstr!(record.target()),
            cstr!(record.args().to_string()),
        );
    }
    fn flush(&self) {}
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        log::trace!(target:$crate::func!(), $($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log::info!(target:$crate::func!(), $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log::warn!(target:$crate::func!(), $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log::error!(target:$crate::func!(), $($arg)*);
    };
}
