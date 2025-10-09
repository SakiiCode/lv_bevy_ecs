#![doc = include_str!("../README.md")]
#![no_std]

use core::ops::{Deref, DerefMut};

use bevy_ecs::{schedule::Schedule, system::Local, world::World};
use embedded_time::{Clock, Instant};

use crate::{functions::lv_tick_inc, widgets::on_insert_parent};

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

struct FrameInstant<C: Clock>(Instant<C>);

impl<C: Clock> Deref for FrameInstant<C> {
    type Target = Instant<C>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C: Clock> DerefMut for FrameInstant<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn lvgl_update<C>(mut prev_time: Local<Option<FrameInstant<C>>>, mut clock: Local<C>)
where
    C: Clock<T = u32> + Default + Send,
    C::T: Send,
{
    if prev_time.is_none() {
        *clock = C::default();
        *prev_time = Some(FrameInstant(clock.try_now().unwrap()));
        return;
    }
    let current_time = clock.try_now().unwrap();
    let diff = current_time
        .checked_duration_since(prev_time.as_ref().unwrap())
        .unwrap();
    *prev_time = Some(FrameInstant(current_time));
    lv_tick_inc(embedded_time::duration::Milliseconds(diff.integer()));
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
    pub fn new<C: Clock<T = u32> + Default + Send + 'static>() -> Schedule {
        let mut schedule = Schedule::default();
        schedule.add_systems(lvgl_update::<C>);
        schedule
    }
}
