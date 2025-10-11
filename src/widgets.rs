//! # Widgets
//!
//! ```rust
//! let mut label: Widget = Label::create_widget()?;
//! lv_label_set_text(&mut label, c"Example label");
//! world.spawn((Label, label));
//! ```
//!
//! To create widgets, you have to use "zero-sized marker structs" (Arc, Label, Button, ...) that create `Widget` objects
//! using the `create_widget` function.
//!
//! Most of the LVGL functions have been kept with original names and they will use this `Widget` as their first parameter.
//!
//! If you need to know the type of the Widget later on, pass the marker struct next to it when spawning the entity.
//! This is not mandatory but useful for queries. If marker structs are omitted, the storage will be slightly better optimized.
//!
//! ## Modifying Widgets
//!
//! To access widgets after moving them to the World with the `spawn()` function, you have to store the created Entity ID or use queries.
//!
//! ```rust
//! let mut label_widget = Label::create_widget();
//! let label_entity = world.spawn((Label, label_widget)).id();
//! let mut label_widget = world.get_mut::<Widget>(label_entity).unwrap();
//! ```
//!
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

use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use bevy_ecs::{
    component::Component, hierarchy::ChildOf, lifecycle::Insert, observer::On, system::Query,
};
use lightvgl_sys::{lv_obj_delete, lv_obj_t};

use crate::trace;

#[derive(Component)]
pub struct Widget {
    raw: NonNull<lightvgl_sys::lv_obj_t>,
}

impl Widget {
    pub fn raw(&self) -> *mut lightvgl_sys::lv_obj_t {
        self.raw.as_ptr()
    }

    pub fn from_raw(ptr: *mut lightvgl_sys::lv_obj_t) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(ptr)?,
        })
    }

    pub fn from_non_null(ptr: NonNull<lightvgl_sys::lv_obj_t>) -> Self {
        Self { raw: ptr }
    }

    pub fn as_wdg(&self) -> &Wdg {
        Wdg::from_non_null(&self.raw)
    }

    pub fn as_mut_wdg(&mut self) -> &mut Wdg {
        Wdg::from_non_null_mut(&mut self.raw)
    }
}

unsafe impl Send for Widget {}
unsafe impl Sync for Widget {}

impl Drop for Widget {
    fn drop(&mut self) {
        unsafe {
            trace!("Dropping Obj");
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
                    let default_screen = lightvgl_sys::lv_display_get_screen_active(
                        lightvgl_sys::lv_display_get_default(),
                    );
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
    trigger: On<Insert, ChildOf>,
    widgets: Query<&Widget>,
    children: Query<(&Widget, &ChildOf)>,
) {
    let event = trigger.event();
    let parent_widget = children.get(event.entity).unwrap();
    let parent_ptr = widgets.get(parent_widget.1.0).unwrap().raw();
    let child_ptr = children.get(event.entity).unwrap().0.raw();
    unsafe {
        lightvgl_sys::lv_obj_set_parent(child_ptr, parent_ptr);
    }
    trace!("On Insert Parent");
}

/// Represents a borrowed Widget
#[repr(transparent)]
pub struct Wdg {
    raw: NonNull<lv_obj_t>,
}

impl Wdg {
    pub fn from_ptr(ptr: *mut lv_obj_t) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(ptr)?,
        })
    }

    /*pub fn from_ref<'a>(mut r#ref: &'a mut lv_obj_t) -> &'a mut Self {
        // this works
        /*Some(Self {
            raw: NonNull::new(r#ref as *mut lv_obj_t)?,
        })*/
        // this does not
        //unsafe { (&mut r#ref as *mut _ as *mut Self).as_mut().unwrap() }
    }*/

    pub fn from_non_null<'a>(ptr: &'a NonNull<lv_obj_t>) -> &'a Self {
        unsafe { &*(ptr as *const _ as *const Self) }
    }

    pub fn from_non_null_mut<'a>(ptr: &'a mut NonNull<lv_obj_t>) -> &'a mut Self {
        unsafe { &mut *(ptr as *mut _ as *mut Self) }
    }

    pub fn raw(&mut self) -> *mut lv_obj_t {
        self.raw.as_ptr()
    }
}

impl Deref for Widget {
    type Target = Wdg;
    fn deref(&self) -> &Self::Target {
        Wdg::from_non_null(&self.raw)
    }
}

impl DerefMut for Widget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Wdg::from_non_null_mut(&mut self.raw)
    }
}

include!(concat!(env!("OUT_DIR"), "/widgets.rs"));
