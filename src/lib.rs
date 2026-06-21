#![cfg_attr(doc, doc = include_str!("../README.md"))]
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
    //! Re-exported modules from `lightvgl_sys`
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

#[cfg(feature = "defmt")]
pub use defmt::debug;
#[cfg(feature = "defmt")]
pub use defmt::error;
#[cfg(feature = "defmt")]
pub use defmt::info;
#[cfg(feature = "defmt")]
pub use defmt::trace;
#[cfg(feature = "defmt")]
pub use defmt::warn;
