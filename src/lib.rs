pub mod animation;
pub mod display;
pub mod styles;
pub mod support;
pub mod widgets;
pub mod input;

pub fn init() {
    unsafe {
        lvgl_sys::lv_init();
    }
}
