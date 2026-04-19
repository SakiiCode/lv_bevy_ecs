#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

#[cfg(feature = "lvgl-alloc")]
pub mod allocator;
pub mod animation;
pub mod bevy {
    //! Re-exported modules from bevy_ecs
    pub use bevy_ecs::*;
}
pub mod display;
pub mod events;
pub mod functions;
pub mod input;
pub mod logging;
#[cfg(feature = "rust-alloc")]
pub mod malloc;
pub mod styles;
pub mod subjects;
pub mod support;
pub mod sys {
    //! Re-exported modules from lightvgl_sys
    pub use lightvgl_sys::*;
}
pub mod timers;
#[macro_use]
pub mod widgets;

#[cfg(feature = "ctor")]
#[ctor::ctor]
unsafe fn init() {
    crate::functions::lv_init();
}

pub use logging::error_ as error;
pub use logging::info_ as info;
pub use logging::trace_ as trace;
pub use logging::warn_ as warn;
