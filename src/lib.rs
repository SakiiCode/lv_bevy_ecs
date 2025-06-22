pub mod animation;
pub mod display;
pub mod styles;
pub mod support;
pub mod widgets;
pub mod input;
pub mod events;
pub mod functions;

pub mod prelude {
    pub use lvgl_sys::*;
    pub use bevy_ecs::*;
}

pub fn init() {
    unsafe {
        lvgl_sys::lv_init();
    }
}

