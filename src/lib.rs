use std::time::Duration;

use bevy_ecs::world::World;

use crate::widgets::on_insert_parent;

#[cfg(feature = "lvgl-alloc")]
mod alloc;
pub mod animation;
pub mod display;
pub mod events;
pub mod functions;
pub mod input;
pub mod styles;
pub mod subjects;
pub mod support;
pub mod timers;
pub mod widgets;

pub mod prelude {
    pub use bevy_ecs::*;
    pub use lvgl_sys::*;
}

pub fn init() {
    unsafe {
        lvgl_sys::lv_init();
    }
}

pub fn lv_tick_inc(diff: Duration) {
    unsafe {
        lvgl_sys::lv_tick_inc(diff.as_millis() as u32);
    }
}

pub fn lv_timer_handler() {
    unsafe {
        lvgl_sys::lv_timer_handler();
    }
}

pub struct LvglWorld;

impl LvglWorld {
    pub fn new() -> World {
        let mut world = World::new();
        world.add_observer(on_insert_parent);
        world
    }
}
