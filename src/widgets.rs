//! # Widgets
//!
//! ```ignore
//! let mut label: Widget = Label::create_widget();
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
//! This is not mandatory but useful for queries. If marker structs are omitted, the storage will be slightly better optimized in memory.
//!
//! ## Modifying Widgets
//!
//! To access widgets after moving them to the World with the `spawn()` function, you have to store the created Entity ID or use queries.
//!
//! ```ignore
//! let mut label_widget = Label::create_widget();
//! let label_entity = world.spawn((Label, label_widget)).id();
//! let mut label_widget = world.get_mut::<Widget>(label_entity).unwrap();
//! ```
//!
//! ```ignore
//! let mut labels = world.query_filtered::<&mut Widget, With<Label>>();
//! for label in labels.iter_mut(){
//!   //...
//! }
//! ```
//!
//! In case of a unique entity:
//! ```ignore
//! let mut label = world.query_filtered::<&mut Widget, With<Label>>().single_mut().unwrap();
//! ```
//!
//! You are free to define any kind of custom component:
//!
//! ```ignore
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
//! ```ignore
//! let mut button_entity = world.spawn((Button, button));
//! let mut label_entity = button_entity.with_child((Label, label));
//! ```

use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::info;
use bevy_ecs::{
    component::Component,
    hierarchy::ChildOf,
    lifecycle::Insert,
    observer::On,
    system::{ParamSet, Query},
    world::World,
};
use lightvgl_sys::{lv_obj_delete, lv_obj_t};

pub struct LvglWorld;

impl LvglWorld {
    pub fn new() -> World {
        let mut world = World::new();
        world.add_observer(on_insert_parent);
        world
    }
}

#[derive(Component, PartialEq)]
pub struct Widget {
    raw: NonNull<lightvgl_sys::lv_obj_t>,
}

impl Widget {
    pub fn raw(&self) -> *const lightvgl_sys::lv_obj_t {
        self.raw.as_ptr()
    }

    pub fn raw_mut(&mut self) -> *mut lightvgl_sys::lv_obj_t {
        self.raw.as_ptr()
    }

    pub fn from_ptr(ptr: *mut lightvgl_sys::lv_obj_t) -> Option<Self> {
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
            info!("Dropping Obj");
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
            /// Creates a widget or panics if LVGL returned a null pointer.
            pub fn create_widget() -> crate::widgets::Widget {
                Self::try_create_widget().expect("Could not create widget")
            }

            /// Creates a widget or returns None if LVGL returned a null pointer.
            pub fn try_create_widget() -> Option<crate::widgets::Widget> {
                unsafe {
                    let default_screen = lightvgl_sys::lv_display_get_screen_active(
                        lightvgl_sys::lv_display_get_default(),
                    );
                    let ptr = $func(default_screen);
                    crate::widgets::Widget::from_ptr(ptr)
                }
            }
        }
    };
}

fn on_insert_parent(
    trigger: On<Insert, ChildOf>,
    mut set: ParamSet<(
        /* widgets */ Query<&mut Widget>,
        /* children */ Query<(&mut Widget, &ChildOf)>,
    )>,
) {
    let event = trigger.event();
    let parent_widget = set.p1().get_mut(event.entity).unwrap().1.0;
    let parent_ptr = set.p0().get_mut(parent_widget).unwrap().raw_mut();
    let child_ptr = set.p1().get_mut(event.entity).unwrap().0.raw_mut();
    unsafe {
        lightvgl_sys::lv_obj_set_parent(child_ptr, parent_ptr);
    }
    info!("On Insert Parent");
}

/// Represents a borrowed Widget
#[derive(PartialEq)]
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

    pub fn raw(&self) -> *const lv_obj_t {
        self.raw.as_ptr()
    }

    pub fn raw_mut(&mut self) -> *mut lv_obj_t {
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
