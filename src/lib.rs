use std::time::Duration;

pub mod animation;
pub mod display;
pub mod styles;
pub mod support;
pub mod widgets;
pub mod input;
pub mod events;
pub mod functions;
pub mod subjects;
pub mod timers;

pub mod prelude {
    pub use lvgl_sys::*;
    pub use bevy_ecs::*;
}

pub fn init() {
    unsafe {
        lvgl_sys::lv_init();
    }
}

pub fn lv_tick_inc(diff: Duration){
    unsafe {
        lvgl_sys::lv_tick_inc(diff.as_millis() as u32);
    }
}

pub fn lv_timer_handler(){
    unsafe {
        lvgl_sys::lv_timer_handler();
    }
}