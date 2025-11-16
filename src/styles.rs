//! # Styles
//!
//! Styles are components that need to be added to entities.
//! Right now a Style can only be applied to a single widget, but they are cloneable.
//!
//! ```rust
//! # use lv_bevy_ecs::functions::*;
//! # use lv_bevy_ecs::styles::Style;
//! # use lv_bevy_ecs::support::OpacityLevel;
//! # use lv_bevy_ecs::sys::lv_part_t_LV_PART_MAIN;
//! # use lv_bevy_ecs::widgets::*;
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! # let mut world = LvglWorld::default();
//! #
//! # let mut button = Button::create_widget();
//! # let mut button_entity = world.spawn((Button, button));
//! #
//! let mut style = Style::default();
//! let opacity = OpacityLevel::Percent50 as u8;
//! lv_style_set_opa(&mut style, opacity);
//!
//! button_entity.insert(style);
//! let widget = button_entity.get_mut::<Widget>().unwrap();
//! assert_eq!(lv_obj_get_style_opa_recursive(&*widget, lv_part_t_LV_PART_MAIN), opacity - 1);
//! ```

use ::core::mem::MaybeUninit;

use bevy_ecs::{component::Component, lifecycle::HookContext, world::DeferredWorld};
use lightvgl_sys::{lv_part_t_LV_PART_MAIN, lv_style_selector_t};

use crate::{functions::lv_style_copy, info, widgets::Widget};

#[derive(Component)]
#[component(on_insert=add_style)]
#[component(on_replace=remove_style)]
#[component(storage = "SparseSet")]
pub struct Style {
    raw: lightvgl_sys::lv_style_t,
    selector: lv_style_selector_t,
}

impl Default for Style {
    fn default() -> Self {
        let raw = unsafe {
            let mut style = MaybeUninit::<lightvgl_sys::lv_style_t>::uninit();
            lightvgl_sys::lv_style_init(style.as_mut_ptr());
            style.assume_init()
        };
        Self {
            raw,
            selector: lv_part_t_LV_PART_MAIN,
        }
    }
}

impl Clone for Style {
    fn clone(&self) -> Self {
        let mut result = Style::default();
        lv_style_copy(&mut result, self);
        result.selector = self.selector;
        result
    }
}

impl Style {
    pub fn raw(&self) -> &lightvgl_sys::lv_style_t {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut lightvgl_sys::lv_style_t {
        &mut self.raw
    }

    pub fn new(selector: lv_style_selector_t) -> Self {
        let raw = unsafe {
            let mut style = MaybeUninit::<lightvgl_sys::lv_style_t>::uninit();
            lightvgl_sys::lv_style_init(style.as_mut_ptr());
            style.assume_init()
        };
        Self { raw, selector }
    }
}

impl Drop for Style {
    fn drop(&mut self) {
        info!("Dropping Style");
    }
}

unsafe impl Send for Style {}
unsafe impl Sync for Style {}

fn add_style(mut world: DeferredWorld, ctx: HookContext) {
    // TODO make this safer
    let widget = world
        .get_mut::<Widget>(ctx.entity)
        .expect("Style components must be added to Widget entities")
        .raw_mut();
    let style = world.get_mut::<Style>(ctx.entity).unwrap();
    unsafe {
        lightvgl_sys::lv_obj_add_style(widget, &style.raw, style.selector);
    }
    info!("Added Style");
}

fn remove_style(mut world: DeferredWorld, ctx: HookContext) {
    // TODO make this safer
    let widget = world
        .get_mut::<Widget>(ctx.entity)
        .expect("Style components must be added to Widget entities")
        .raw_mut();
    let style = world.get_mut::<Style>(ctx.entity).unwrap();
    unsafe {
        lightvgl_sys::lv_obj_remove_style(widget, &style.raw, style.selector);
    }
    info!("Removed Style");
}
