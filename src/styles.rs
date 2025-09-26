//! # Styles
//!
//! Styles are components that need to be added to entities.
//! Right now a Style can be only applied to a single widget.
//!
//! ```rust
//! let mut button = Button::create_widget()?;
//! let mut button_entity = world.spawn((Button, button));
//!
//! let mut style = Style::default();
//! lv_style_set_opa(&mut style, LV_OPA_50 as u8);
//!
//! button_entity.insert(style);
//! ```

use bevy_ecs::{
    component::{Component, HookContext},
    world::DeferredWorld,
};
use lightvgl_sys::{LV_PART_MAIN, lv_style_selector_t};

use crate::widgets::Widget;

#[derive(Component, Clone)]
#[component(on_insert=add_style)]
#[component(on_replace=remove_style)]
pub struct Style {
    raw: lightvgl_sys::lv_style_t,
    selector: lv_style_selector_t,
}

impl Default for Style {
    fn default() -> Self {
        let raw = unsafe {
            let mut style = std::mem::MaybeUninit::<lightvgl_sys::lv_style_t>::uninit();
            lightvgl_sys::lv_style_init(style.as_mut_ptr());
            style.assume_init()
        };
        Self {
            raw,
            selector: LV_PART_MAIN,
        }
    }
}

impl Style {
    pub fn raw(&mut self) -> &mut lightvgl_sys::lv_style_t {
        &mut self.raw
    }

    pub fn new(selector: lv_style_selector_t) -> Self {
        let raw = unsafe {
            let mut style = std::mem::MaybeUninit::<lightvgl_sys::lv_style_t>::uninit();
            lightvgl_sys::lv_style_init(style.as_mut_ptr());
            style.assume_init()
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

fn add_style(mut world: DeferredWorld, ctx: HookContext) {
    // TODO make this safer
    let widget = world
        .get_mut::<Widget>(ctx.entity)
        .expect("Style components must be added to Widget entities")
        .raw();
    let mut style = world.get_mut::<Style>(ctx.entity).unwrap();
    unsafe {
        lightvgl_sys::lv_obj_add_style(widget, &mut style.raw, style.selector);
    }
    dbg!("Added style");
}

fn remove_style(mut world: DeferredWorld, ctx: HookContext) {
    // TODO make this safer
    let widget = world
        .get_mut::<Widget>(ctx.entity)
        .expect("Style components must be added to Widget entities")
        .raw();
    let mut style = world.get_mut::<Style>(ctx.entity).unwrap();
    unsafe {
        lightvgl_sys::lv_obj_remove_style(widget, &mut style.raw, style.selector);
    }
    dbg!("Removed style");
}
