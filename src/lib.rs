#![doc = include_str!("../README.md")]

use std::{
    ops::{Deref, DerefMut},
    time::Instant,
};

use bevy_ecs::{schedule::Schedule, system::Local, world::World};

use crate::{functions::lv_tick_inc, widgets::on_insert_parent};

#[cfg(feature = "lvgl-alloc")]
mod alloc;

#[macro_use]
pub mod widgets;
pub mod animation;
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

struct FrameInstant(Instant);

impl Default for FrameInstant {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl Deref for FrameInstant {
    type Target = Instant;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FrameInstant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn lvgl_update(mut prev_time: Local<FrameInstant>) {
    let current_time = Instant::now();
    let diff = current_time.duration_since(**prev_time);
    **prev_time = current_time;
    lv_tick_inc(diff);
}

pub struct LvglWorld;

impl LvglWorld {
    pub fn new() -> World {
        let mut world = World::new();
        world.add_observer(on_insert_parent);
        world
    }
}

pub struct LvglSchedule;

impl LvglSchedule {
    pub fn new() -> Schedule {
        let mut schedule = Schedule::default();
        schedule.add_systems(lvgl_update);
        schedule
    }
}
