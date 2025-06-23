use bevy_ecs::{
    component::{Component, HookContext},
    world::DeferredWorld,
};
use lvgl_sys::{LV_PART_MAIN, lv_style_selector_t};

use crate::widgets::Widget;

#[derive(Component, Clone)]
#[component(on_insert=add_style)]
#[component(on_replace=remove_style)]
pub struct Style {
    raw: Box<lvgl_sys::lv_style_t>,
    selector: lv_style_selector_t,
}

impl Default for Style {
    fn default() -> Self {
        let raw = unsafe {
            let mut style = std::mem::MaybeUninit::<lvgl_sys::lv_style_t>::uninit();
            lvgl_sys::lv_style_init(style.as_mut_ptr());
            Box::new(style.assume_init())
        };
        Self {
            raw,
            selector: LV_PART_MAIN,
        }
    }
}

impl Style {
    pub fn raw(&mut self) -> *mut lvgl_sys::lv_style_t {
        self.raw.as_mut()
    }

    pub fn new(selector: lv_style_selector_t) -> Self {
        let raw = unsafe {
            let mut style = std::mem::MaybeUninit::<lvgl_sys::lv_style_t>::uninit();
            lvgl_sys::lv_style_init(style.as_mut_ptr());
            Box::new(style.assume_init())
        };
        Self { raw, selector }
    }
}

impl Drop for Style {
    fn drop(&mut self) {
        dbg!("Dropping style");
    }
}

unsafe impl Send for Style {}
unsafe impl Sync for Style {}

pub fn add_style(mut world: DeferredWorld, ctx: HookContext) {
    // TODO make this safer
    let widget = world
        .get_mut::<Widget>(ctx.entity)
        .expect("Style components must be added to Widget entities")
        .raw();
    let mut style = world.get_mut::<Style>(ctx.entity).unwrap();
    unsafe {
        lvgl_sys::lv_obj_add_style(widget, style.raw.as_mut(), style.selector);
    }
    dbg!("Added style");
}

pub fn remove_style(mut world: DeferredWorld, ctx: HookContext) {
    // TODO make this safer
    let widget = world
        .get_mut::<Widget>(ctx.entity)
        .expect("Style components must be added to Widget entities")
        .raw();
    let mut style = world.get_mut::<Style>(ctx.entity).unwrap();
    unsafe {
        lvgl_sys::lv_obj_remove_style(widget, style.raw.as_mut(), style.selector);
    }
    dbg!("Removed style");
}
