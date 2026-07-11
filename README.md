# lv_bevy_ecs

Safe Rust bindings to the LVGL library using bevy_ecs. Compatible with `#![no_std]` environments by default. An `alloc` implementation is required.

[![Crates.io](https://img.shields.io/crates/v/lv_bevy_ecs.svg)](https://crates.io/crates/lv_bevy_ecs)
[![Docs](https://docs.rs/lv_bevy_ecs/badge.svg)](https://docs.rs/lv_bevy_ecs/latest/lv_bevy_ecs/)
[![Changelog](https://img.shields.io/badge/changelog-.md-darkorchid)](https://github.com/SakiiCode/lv_bevy_ecs/blob/master/CHANGELOG.md)
![License](https://img.shields.io/badge/license-MIT-informational.svg)

> [!NOTE]
> This crate is under heavy development and the API has not settled yet. Expect several breaking changes in every 0.x release.
> Check the [CHANGELOG.md](https://github.com/SakiiCode/lv_bevy_ecs/blob/master/CHANGELOG.md) for help with migration.

## What is an ECS?

ECS stands for Entity Component System. You can think of it as a database with rows (entities),
columns (components) and jobs (systems).

You have to move LVGL objects into this database,
so that they don't go out of scope and get deallocated. Bevy's Observers will mirror these database operations to LVGL's world.

### But I don't want to use an ECS...

Enabling the `no_ecs` feature unlocks some functions that allow you to bring your own storage solution.

If you don't care about storage at all, and know in advance that a Widget will live for the rest of the program's execution,
you can call `Widgetclass::leak(mywidget)` or `Widget::leak(mywidget.into_inner())` to leak memory and prevent calling the destructor.

Check out [no_ecs.rs](https://github.com/SakiiCode/lv_bevy_ecs/blob/master/examples/no_ecs.rs) on how to use these.

## Features

- [x] Displays
- [x] Widgets
- [x] Events
- [x] Styles
- [x] Input devices
- [x] Animations
- [x] Timers, lv_async_call
- [x] Subjects
- [x] Logging
- [x] LVGL allocator
- [x] "no_ecs" mode
- [x] #![no_std] compatibility
- [x] LVGL docstrings
- [x] Cross-language LTO support
- [x] Defmt support
- [x] OOP-style function calls
- [ ] Auto-generated enums
- [ ] File system
- [ ] Custom fonts
- [ ] Snapshots
- [ ] Non-widget functions
- [ ] Layouts
- [ ] XML UI

## Usage

Check the [documentation](https://docs.rs/crate/lv_bevy_ecs/latest) and the [examples](https://github.com/SakiiCode/lv_bevy_ecs/tree/master/examples).

## Running the demos

```sh
sudo apt install libsdl2-dev

git clone git@github.com:SakiiCode/lv_bevy_ecs.git
cd lv_bevy_ecs
cargo run --example basic
```

On Windows, you have to [install SDL2](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc).

## Building for embedded

Example projects are available targeting ESP32 and ESP32-P4: [lvgl-bevy-demo](https://github.com/SakiiCode/lvgl-bevy-demo), [lvgl-bevy-demo-dsi](https://github.com/SakiiCode/lvgl-bevy-demo-dsi)

### Heap Allocation

#### `lvgl-alloc` feature

If you don't have an allocator, a [GlobalAlloc](https://github.com/SakiiCode/lv_bevy_ecs/blob/master/src/allocator.rs) for Rust leveraging the [LVGL memory allocator](https://docs.lvgl.io/9.5/API/stdlib/lv_mem_h.html) is provided, but not enabled by default.
Can be enabled with the feature `lvgl-alloc`. This will make all dynamic memory to be allocated by LVGL internal memory manager.

#### `rust-alloc` feature

If you already have an allocator, you can enable the `rust-alloc` feature to forward the LVGL memory allocator functions to the Rust `alloc` crate.
This needs `LV_USE_STDLIB_MALLOC` set to `LV_STDLIB_CUSTOM` in `lv_conf.h`.

Additionally, an optional implementation of the `get_memory_stats(&mut lv_mem_monitor_t)` function can be provided.
Check the examples and sample projects for reference implementation.

## Compatibility table

| lv_bevy_ecs | bevy_ecs | lightvgl-sys |
| ----------- | -------- | ------------ |
| 0.12.x      | 0.19.x   | 9.5.x        |
| 0.11.x      | 0.18.x   | 9.5.x        |
| 0.10.x      | 0.18.x   | 9.5.x        |
| 0.9.x       | 0.18.x   | 9.5.x        |
| 0.8.x       | 0.18.x   | 9.5.x        |
| 0.7.x       | 0.18.x   | 9.4.x        |
| 0.6.x       | 0.17.x   | 9.4.x        |
| 0.5.x       | 0.17.x   | 9.4.x        |
| 0.4.x       | 0.17.x   | 9.3.x        |
| 0.3.x       | 0.16.x   | 9.3.x        |
| 0.2.x       | 0.16.x   | 9.2.x        |

## Contributing

Feel free to open issues for features you find important and missing. I am not completely satisfied with the API,
so open to API improvement ideas as well.

## Troubleshooting

### Unable to generate bindings: fatal error: 'inttypes.h' file not found

Check if your bindgen sysroot is correctly set with the `LV_SYSROOT` environment variable.

## Thanks

This project heavily builds upon the work in the the original [lv_binding_rust](https://github.com/lvgl/lv_binding_rust) repo.
