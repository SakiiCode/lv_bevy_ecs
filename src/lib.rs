#![doc = include_str!("../README.md")]
#![no_std]

use bevy_ecs::world::World;

use crate::widgets::on_insert_parent;

#[cfg(not(feature = "lvgl-alloc"))]
extern crate alloc;

#[cfg(feature = "lvgl-alloc")]
mod alloc;

#[macro_use]
pub mod widgets;
pub mod animation;
pub mod clock;
pub mod display;
pub mod events;
pub mod functions;
pub mod input;
pub mod logging;
pub mod styles;
pub mod subjects;
pub mod support;
pub mod timers;

pub mod prelude {
    //! Re-exported modules from bevy_ecs and lightvgl_sys
    pub use bevy_ecs::*;
    pub use lightvgl_sys::*;
}

#[cfg(feature = "ctor")]
#[ctor::ctor]
fn init() {
    crate::functions::lv_init();
}

pub struct LvglWorld;

impl LvglWorld {
    pub fn new() -> World {
        let mut world = World::new();
        world.add_observer(on_insert_parent);
        world
    }
}
