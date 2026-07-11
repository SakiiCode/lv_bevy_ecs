#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![allow(clippy::needless_doctest_main)]
//! ## Usage
//!
//!It is highly recommended to read [Chapter 14 of the Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/programming.html) before using this library.
//!
//!1.  Create a project with `cargo new` or `esp-generate`, then
//!
//!```sh
//!cargo add lv_bevy_ecs
//! ```
//!
//! 2.  This package depends on [lightvgl-sys](https://crates.io/crates/lightvgl-sys) to generate the raw unsafe bindings.
//!     It needs an environment variable called `DEP_LV_CONFIG_PATH` that specifies the path to the folder containing `lv_conf.h` file.
//!
//!     It is recommended to add it inside `.cargo/config.toml`
//!
//! ```toml
//! [env]
//! DEP_LV_CONFIG_PATH = { relative = true, value = "." }
//! ```
//!
//! 3. Assign a tick callback that measures elapsed time in milliseconds. This must be done **before** creating the UI.
//!    _For other frameworks (like ESP-IDF or Embassy), you should [use its tick counter](https://docs.lvgl.io/9.5/integration/overview.html#tick-interface) instead to get precise and constant framerate._
//!
//! ```rust
//! # use lv_bevy_ecs::functions::lv_tick_set_cb;
//! # use std::time::Instant;
//! #
//! let start = Instant::now();
//! lv_tick_set_cb(move || start.elapsed().as_millis() as u32);
//! ```
//!
//! 4. Create a global LvglWorld instance with [`LvglWorld::new()`](crate::widgets::LvglWorld::new):
//!    - **Option 1:** Lazy-initialized global Mutex
//!
//! ```rust
//! # use std::sync::{LazyLock, Mutex};
//! # use lv_bevy_ecs::widgets::LvglWorld;
//! #
//! static WORLD: LazyLock<Mutex<LvglWorld>> = LazyLock::new(|| Mutex::new(LvglWorld::new()));
//! ```
//!
//! - **Option 2:** `Rc<RefCell<T>>` or `Arc<Mutex<T>>`
//!
//! ```rust
//! # use std::rc::Rc;
//! # use std::cell::RefCell;
//! # use lv_bevy_ecs::events::EventCode;
//! # use lv_bevy_ecs::widgets::*;
//! #
//! fn main(){
//! #   lv_bevy_ecs::setup_test_display!();
//! #
//!     let world_rc = Rc::new(RefCell::new(LvglWorld::new()));
//!     some_other_function(world_rc.clone());
//! }
//!
//! fn some_other_function(world_rc: Rc<RefCell<LvglWorld>>){
//!     let world = world_rc.clone();
//!     let mut button = Button::new();
//!     button.add_event_cb(EventCode::Pressed, move |event| {
//!         let label = Label::new();
//!         world.borrow_mut().spawn(label.into_inner());
//!     });
//!
//! }
//! ```
//!
//! - **Option 3**: Manually initialized global Mutex
//!
//! ```rust
//! # use std::sync::Mutex;
//! # use lv_bevy_ecs::widgets::UnsafeLvglWorld;
//! #
//! static WORLD: Mutex<UnsafeLvglWorld> = Mutex::new(UnsafeLvglWorld::new());
//!
//! fn main(){
//!     // Make sure to run `.init()` before the first use!
//!     WORLD.lock().unwrap().init();
//! }
//! ```
//!
//! 5. The last step is to call [`lv_timer_handler()`](crate::functions::lv_timer_handler) periodically.
//!
//! ```rust
//! # use lv_bevy_ecs::functions::*;
//! # use std::thread::sleep;
//! # use std::time::{Duration, Instant};
//! #
//! loop {
//!     let start = Instant::now();
//!     let next_timer_period = lv_timer_handler();
//! #   break;
//!     match next_timer_period {
//!         NextTimerPeriod::Ready => {
//!             // yield or continue
//!             continue;
//!         }
//!         NextTimerPeriod::AfterMs(next_timer_ms) => {
//!             let next_instant = start + Duration::from_millis(next_timer_ms.get().into());
//!             sleep(next_instant - Instant::now());
//!         }
//!         NextTimerPeriod::Never => {
//!             sleep(Duration::from_secs(5));
//!         }
//!     }
//! }
//! ```
//!
//! ## Minimizing binary size
//!
//! In order to remove even more unused functions, the [Cross-language Link-Time Optimization](https://doc.rust-lang.org/rustc/linker-plugin-lto.html) functionality of LLVM can be enabled. Unfortunately, this is not available on every platform, especially on those that use `gcc` as the linker.
//! Make sure to match your clang version with your rustc version.
//!
//! `.cargo/config.toml`
//!
//! ```toml
//! [target.YOUR-TARGET-TRIPLE]
//! rustflags = ["-C", "linker-plugin-lto", "-C", "link-arg=-fuse-ld=lld"]
//! linker = "clang-21"
//!
//! [env]
//! LV_COMPILE_ARGS = "-flto=full"
//! CC = "clang-21"
//! ```
//!
//! `Cargo.toml`
//!
//! ```toml
//! [profile.dev]
//! opt-level = "z"
//! lto = "fat"
//! codegen-units = 1
//! ```
#![no_std]

extern crate alloc;

pub use bevy_ecs as bevy;
pub use lightvgl_sys as sys;

#[cfg(feature = "lvgl-alloc")]
pub mod allocator;
pub mod animation;
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
