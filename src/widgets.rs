//! # Widgets
//!
//! ```rust
//! let mut label: Widget = Label::create_widget()?;
//! lv_label_set_text(&mut label, cstr!("Example label"));
//! world.spawn((Label, label));
//! ```
//!
//! To create widgets, you have to use "zero-sized marker structs" (Arc, Label, Button, ...) that create `Widget` objects
//! using the `create_widget` function.
//!
//! Most of the LVGL functions have been kept with original names and they will use this `Widget` as their first parameter.
//!
//! In order to ECS to know the type of the Widget, pass the marker struct next to it when spawning the entity. (This is not mandatory but useful for queries)
//!
//! ## Modifying Widgets
//!
//! To access widgets after moving them to the World with the `spawn()` function, you have to use queries
//! ```rust
//! let mut labels = world.query_filtered::<&mut Widget, With<Label>>();
//! for label in labels.iter_mut(){
//!   //...
//! }
//! ```
//!
//! In case of a unique entity:
//! ```rust
//! let mut label = world.query_filtered::<&mut Widget, With<Label>>().single_mut().unwrap();
//! ```
//!
//! You are free to define any kind of custom component:
//!
//! ```rust
//! #[derive(Component)]
//! struct DynamicLabel;
//! // ...
//! world.spawn((Label, label, DynamicLabel));
//! //...
//! let mut label = world.query_filtered::<&mut Widget, With<DynamicLabel>>().single_mut().unwrap();
//! ```
//!
//! ## Child widgets
//! To add a widget as a child, set it as child entity
//! ```rust
//! let mut button_entity = world.spawn((Button, button));
//! let mut label_entity = button_entity.with_child((Label, label));
//! ```

use std::ptr::NonNull;

use bevy_ecs::{
    component::Component, hierarchy::ChildOf, observer::Trigger, system::Query, world::OnInsert,
};
use lvgl_sys::lv_obj_delete;

#[derive(Component)]
pub struct Widget {
    raw: NonNull<lvgl_sys::lv_obj_t>,
}

impl Widget {
    pub fn raw(&self) -> *mut lvgl_sys::lv_obj_t {
        self.raw.as_ptr()
    }

    pub fn from_raw(ptr: *mut lvgl_sys::lv_obj_t) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(ptr)?,
        })
    }

    pub fn from_non_null(ptr: NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self { raw: ptr }
    }
}

unsafe impl Send for Widget {}
unsafe impl Sync for Widget {}

impl Drop for Widget {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping Obj");
            lv_obj_delete(self.raw.as_ptr());
        }
    }
}

macro_rules! impl_widget {
    ($t:ident, $func:path) => {
        #[derive(bevy_ecs::component::Component)]
        pub struct $t;

        impl $t {
            #[allow(dead_code)]
            pub fn create_widget() -> Result<crate::widgets::Widget, crate::support::LvError> {
                unsafe {
                    let default_screen =
                        lvgl_sys::lv_display_get_screen_active(lvgl_sys::lv_display_get_default());
                    let ptr = $func(default_screen);
                    if let Some(raw) = core::ptr::NonNull::new(ptr) {
                        Ok(crate::widgets::Widget::from_non_null(raw))
                    } else {
                        Err(crate::support::LvError::InvalidReference)
                    }
                }
            }
        }
    };
}

pub fn on_insert_parent(
    trigger: Trigger<OnInsert, ChildOf>,
    widgets: Query<&Widget>,
    children: Query<(&Widget, &ChildOf)>,
) {
    let parent_widget = children.get(trigger.target()).unwrap();
    let parent_ptr = widgets.get(parent_widget.1.0).unwrap().raw();
    let child_ptr = children.get(trigger.target()).unwrap().0.raw();
    unsafe {
        lvgl_sys::lv_obj_set_parent(child_ptr, parent_ptr);
    }
    dbg!("On Insert Parent");
}

include!(concat!(env!("OUT_DIR"), "/widgets.rs"));