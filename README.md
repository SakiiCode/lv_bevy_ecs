# lv_bevy_ecs

Safe Rust bindings to the LVGL library using bevy_ecs. Compatible with `#![no_std]` environments after setting `default-features = false`.

[![Crates.io](https://img.shields.io/crates/v/lv_bevy_ecs.svg)](https://crates.io/crates/lv_bevy_ecs)
[![Docs](https://docs.rs/lv_bevy_ecs/badge.svg)](https://docs.rs/lv_bevy_ecs/latest/lv_bevy_ecs/)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## What is an ECS?

ECS stands for Entity Component System. You can think of it as a database with rows (entities),
columns (components) and jobs (systems).

You have to move LVGL objects into this database,
so that they don't go out of scope and get deallocated. Bevy's Observers will mirror these database operations to LVGL's world.

### But I don't want to use an ECS...

Enabling the `no_ecs` feature unlocks some functions that allow you to bring your own storage solution.

If you don't care about storage at all, and know in advance that a Widget will live for the rest of the program's execution,
you can call `Widget::leak()` to leak memory and prevent calling the destructor.

Check out [no_ecs.rs]() on how to use these.

## Usage

It is highly recommended to read [Chapter 14 of the Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/programming.html) before using this library.

1.  Create a project with `cargo new` or `esp-generate`, then

```sh
        cargo add lv_bevy_ecs
```

2.  This package depends on [lightvgl-sys](https://github.com/SakiiCode/lightvgl-sys) to generate the raw unsafe bindings.
    It needs an environment variable called `DEP_LV_CONFIG_PATH` that specifies the path to the folder containing `lv_conf.h` file.

    It is recommended to put it into `.cargo/config.toml`

```toml
[env]
DEP_LV_CONFIG_PATH = { relative = true, value = "." }
```

3. Assign a tick callback that measures elapsed time in milliseconds. This must be done **before** creating the UI.
   _For other frameworks (like FreeRTOS), you should [use its tick counter](https://docs.lvgl.io/9.4/details/integration/overview/connecting_lvgl.html#tick-interface) instead to get precise and constant framerate._

```rust
# use lv_bevy_ecs::functions::*;
# use std::time::{SystemTime, UNIX_EPOCH, Duration};
#
lv_tick_set_cb(|| {
    let current_time = SystemTime::now();
    let diff = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time should only go forward");
    diff.as_millis() as u32
});
```

4. You have to obtain a World instance with `LvglWorld::default();`.
   This is a global variable, it can be stored in a LazyLock or passed around in an Arc<Mutex<>> if needed elsewhere than in main().

```rust
# use lv_bevy_ecs::widgets::LvglWorld;
# use std::sync::{LazyLock, Mutex};

static WORLD: LazyLock<Mutex<LvglWorld>> = LazyLock::new(|| Mutex::new(LvglWorld::default()));
```

4. Last thing is to call `lv_timer_handle()` in every loop cycle.

```rust
# use lv_bevy_ecs::functions::*;
loop {
    lv_timer_handler();
#    break;
}
```

Check the [documentation](https://docs.rs/crate/lv_bevy_ecs/latest) and the [examples](https://github.com/SakiiCode/lv_bevy_ecs/tree/master/examples) for further usage.

## Running the demo

```sh
sudo apt install libsdl2-dev

git clone git@github.com:SakiiCode/lv_bevy_ecs.git
cd lv_bevy_ecs
cargo run --example basic
```

## Building for embedded

There is an example project targeting the Cheap Yellow Display (ESP32) with `std` enabled: [lvgl-bevy-demo](https://github.com/SakiiCode/lvgl-bevy-demo)

### LVGL Global Allocator

A [global allocator](https://github.com/SakiiCode/lv_bevy_ecs/blob/master/src/allocator.rs) for Rust leveraging the [LVGL memory allocator](https://docs.lvgl.io/9.4/API/stdlib/lv_mem_h.html) is provided, but not enabled by default.
Can be enabled with the feature `lvgl_alloc`. This will make all dynamic memory to be allocated by LVGL internal memory manager.

## Features

- [x] Displays
- [x] Widgets
- [x] Widget functions
- [x] Events
- [x] Styles
- [x] Input devices
- [x] Animations
- [x] Timers
- [x] Async calls
- [x] Subjects
- [x] Logging
- [x] LVGL allocator
- [x] "no_ecs" mode
- [x] #![no_std] compatibility
- [x] LVGL docstrings
- [ ] Auto-generated enums
- [ ] File system
- [ ] Custom fonts
- [ ] Snapshots
- [ ] Non-widget functions
- [ ] Layouts
- [ ] XML UI

## Compatibility table

| lv_bevy_ecs | bevy_ecs | lightvgl-sys | LVGL  |
| ----------- | -------- | ------------ | ----- |
| 0.7.0       | 0.18.0   | 9.4.4        | 9.4.0 |
| 0.6.2       | 0.17.3   | 9.4.3        | 9.4.0 |
| 0.6.0       | 0.17.3   | 9.4.2        | 9.4.0 |
| 0.5.2       | 0.17.3   | 9.4.2        | 9.4.0 |
| 0.5.0       | 0.17.2   | 9.4.0        | 9.4.0 |
| 0.4.0       | 0.17.2   | 9.3.0        | 9.3.0 |
| 0.3.0       | 0.16.0   | 9.3.0        | 9.3.0 |
| 0.2.0       | 0.16.0   | 9.2.0        | 9.2.2 |

## Contributing

Feel free to open issues for features you find important and missing. I am not completely satisfied with the API,
so open to API improvement ideas as well.

## Troubleshooting

### #\[ctor\]/#\[dtor\] is not supported on the current target

You are probably on RISC-V. Please help your architecture get upstreamed into [rust-ctor](https://github.com/mmastrac/rust-ctor).
Until then set `default-features = false` and manually call `lv_init();` in the main function.

### Can't find crate for `std` required by `dtor` because it does not declare `#![no_std]`

Set `default-features = false` and manually call `lv_init();` in the main function.

### Unable to generate bindings: fatal error: 'inttypes.h' file not found

Try adding this environment variable to `.cargo/config.toml`:

```toml
BINDGEN_EXTRA_CLANG_ARGS = "-I/usr/include"
```

## Thanks

This project heavily builds upon the work in the the original [lv_binding_rust](https://github.com/lvgl/lv_binding_rust) repo.
