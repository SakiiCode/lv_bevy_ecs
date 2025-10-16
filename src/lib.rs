#![doc = include_str!("../README.md")]

#[macro_use]
pub mod widgets;
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
pub mod styles;
pub mod subjects;
pub mod support;
pub mod sys {
    //! Re-exported modules from lightvgl_sys
    pub use lightvgl_sys::*;
}
pub mod timers;

#[cfg(feature = "ctor")]
#[ctor::ctor]
fn init() {
    crate::functions::lv_init();
}
