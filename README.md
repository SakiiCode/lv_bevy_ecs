# lv_bevy_ecs

Safe Rust bindings to the LVGL library using bevy_ecs.

## What is an ECS?

ECS stands for Entity Component System. You can think of it as a database with rows (entities),
columns (components) and jobs (systems).

You have to move LVGL objects into this database,
so that they don't go out of scope and get deallocated. Bevy's Observers will mirror these database operations to LVGL's world.

## Usage

It is highly recommended to read [Chapter 14 of the Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/programming.html) before using this library.

1.  Create a project with `cargo new` or `esp-generate`, then

        cargo add lv_bevy_ecs

2.  This package depends on [lightvgl-sys](https://github.com/SakiiCode/lightvgl-sys) to generate the raw unsafe bindings.
    It needs an environment variable called `DEP_LV_CONFIG_PATH` that specifies the path to the folder containing `lv_conf.h` file.

    It is recommended to put it into `.cargo/config.toml`

```toml
[env]
DEP_LV_CONFIG_PATH = { relative = true, value = "." }
```

3. You have to obtain a World instance with `LvglWorld::new();`.
   This is a global variable, it can be stored in lazy_static! or passed around in an Arc<Mutex<>> if needed elsewhere than in main().

```rust
lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(LvglWorld::new());
}
```

5. Last thing is a Schedule instance with `LvglSchedule::new()`. Then call it in every loop cycle

```rust
let schedule = LvglSchedule::new();
// ...
loop {
    // ...
    schedule.run(&mut world);
    lv_timer_handler();
}

```

Check the respective module documentations and the examples for further usage.

## Running the demo

```sh
sudo apt install libsdl2-dev

git clone git@github.com:SakiiCode/lv_bevy_ecs.git
cd lv_bevy_ecs
cargo run --example basic
```

## Building for embedded

This package has been tested with ESP32 only.

You need three more env variables in config.toml and the PATH applied from ~/export-esp.sh

```
LIBCLANG_PATH="..."
CROSS_COMPILE="xtensa-esp32-elf"
BINDGEN_EXTRA_CLANG_ARGS="--sysroot ..."
```

`LIBCLANG_PATH` can be found in ~/export-esp.sh

`BINDGEN_EXTRA_CLANG_ARGS` sysroot can be found with `xtensa-esp32-elf-ld --print-sysroot`

### LVGL Global Allocator

A [global allocator](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) for Rust leveraging the [LVGL memory allocator](https://github.com/lvgl/lvgl/blob/master/src/misc/lv_mem.h) is provided, but not enabled by default.
Can be enabled by the feature lvgl-alloc. This will make all dynamic memory to be allocated by LVGL internal memory manager.

### Partitions

It can happen that the project does not fit in the default main partition. To fix that you need to generate a partitions.csv with

```sh
cargo espflash partition-table -o partitions.csv --to-csv target/xtensa-esp32-espidf/release/partition-table.bin
```

and increase the `factory` partition size.

Then add this to `espflash.toml`:

```toml
[idf]
partition_table = "partitions.csv"
```

### Upload speed

To increase upload speed set `baudrate = 460800` in `espflash.toml`

## Features

- [x] Displays
- [x] Widgets
- [x] Events
- [x] Styles
- [x] Input devices
- [x] Animations
- [x] Timers
- [x] Async calls
- [x] Subjects
- [ ] Auto-generated enums
- [ ] Copy C docs to rustdoc
- [ ] #![no_std] compatibility
- [ ] File system
- [ ] Custom fonts
- [ ] Snapshots
- [ ] Some widget functions
- [ ] Layouts
- [ ] XML UI

## Compatibility table

| lv_bevy_ecs   | bevy_ecs | lightvgl-sys | LVGL  |
| ------------- | -------- | ------------ | ----- |
| 0.4 (planned) | 0.17.x   | 9.3.x        | 9.3.0 |
| 0.3           | 0.16.x   | 9.3.x        | 9.3.0 |
| 0.2           | 0.16.x   | 9.2.x        | 9.2.2 |

## Contributing

Feel free to open issues for features you find important and missing. I am not completely satisfied with the API,
so open to API improvement ideas as well.

## Troubleshooting

### #[ctor]/#[dtor] is not supported on the current target

You are probably on RISC-V. Please help your architecture get upstreamed into [rust-ctor](https://github.com/mmastrac/rust-ctor).
Until then set `default-features = false` and manually call `lv_init();` in the main function.

## Thanks

This project heavily builds upon the work in the the original [lv_binding_rust](https://github.com/lvgl/lv_binding_rust) repo.
