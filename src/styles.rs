use std::ops::{Deref, DerefMut};

use bevy_ecs::{
    component::{Component, HookContext},
    world::DeferredWorld,
};
use lvgl_sys::LV_PART_MAIN;

use crate::widgets::Widget;

#[derive(Component, Clone)]
#[component(on_insert=add_style)]
#[component(on_replace=remove_style)]
pub struct Style {
    pub raw: Box<lvgl_sys::lv_style_t>,
}

impl Deref for Style {
    type Target = lvgl_sys::lv_style_t;
    fn deref(&self) -> &Self::Target {
        self.raw.as_ref()
    }
}

impl DerefMut for Style {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.raw.as_mut()
    }
}

impl Default for Style {
    fn default() -> Self {
        let raw = unsafe {
            let mut style = std::mem::MaybeUninit::<lvgl_sys::lv_style_t>::uninit();
            lvgl_sys::lv_style_init(style.as_mut_ptr());
            Box::new(style.assume_init())
        };
        Self { raw }
    }
}

unsafe impl Send for Style {}
unsafe impl Sync for Style {}

pub fn add_style(mut world: DeferredWorld, ctx: HookContext) {
    // TODO make this safer
    let widget = world
        .get_mut::<Widget>(ctx.entity)
        .expect("Style components must be added to Widget entities")
        .raw
        .as_ptr();
    let mut style = world.get_mut::<Style>(ctx.entity).unwrap();
    unsafe {
        lvgl_sys::lv_obj_add_style(widget, style.raw.as_mut(), LV_PART_MAIN);
    }
    dbg!("Added style");
}

pub fn remove_style(mut world: DeferredWorld, ctx: HookContext) {
    // TODO make this safer
    let widget = world
        .get_mut::<Widget>(ctx.entity)
        .expect("Style components must be added to Widget entities")
        .raw
        .as_ptr();
    let mut style = world.get_mut::<Style>(ctx.entity).unwrap();
    unsafe {
        lvgl_sys::lv_obj_remove_style(widget, style.raw.as_mut(), LV_PART_MAIN);
    }
    dbg!("Removed style");
}
