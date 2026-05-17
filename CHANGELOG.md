# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.0] - 2026-05-17

### Breaking changes

- Renamed `create()` constructors to `new()`
- Safe LVGL wrapper functions have been changed to OOP style
- `lv_timer_handler` now returns `NextTimerPeriod`
- `events::Event` has been renamed to `events::EventCode`
- Reworked widget structs to be generic over `<Widget>` or `<Wdg>` ([docs](https://docs.rs/lv_bevy_ecs/0.10.0-beta.1/lv_bevy_ecs/widgets/))
- `get_memory_stats` is now optional, needs to be be registered using `set_mem_monitor()`

### Added

- `display` field to the `DisplayRefresh` object, borrowing it from the outside is no longer needed
- `UnsafeLvglWorld` to make it easier to work with static globals
- `demos` feature to compile demos when `LV_BUILD_DEMOS 1` is enabled
- New example (`demos.rs`) to test the LVGL Widgets Demo
- `display.register_raw(...)` function to use an arbitrary `&mut [u8]` slice as buffer
- Added color format checks
- Every input method is now implemented

### Fixed

- Improved Windows support
