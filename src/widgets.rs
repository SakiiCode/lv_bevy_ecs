//! # Widgets
//!
//! ```
//! # use lv_bevy_ecs::widgets::*;
//! # use lv_bevy_ecs::functions::*;
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! let mut world = LvglWorld::default();
//! let mut label: Widget = Label::create_widget();
//! lv_label_set_text(&mut label, c"Example label");
//! let mut label_entity = world.spawn((Label, label));
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
//! ```
//! # use core::ffi::CStr;
//! # use lv_bevy_ecs::widgets::{Widget, Label, LvglWorld};
//! # use lv_bevy_ecs::functions::*;
//! # use lv_bevy_ecs::sys::lv_label_get_text;
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! # let mut world = LvglWorld::default();
//! # let mut label: Widget = Label::create_widget();
//! # lv_label_set_text(&mut label, c"Example label");
//! # let mut label_entity = world.spawn((Label, label));
//! #
//! let mut label_widget = label_entity.get_mut::<Widget>().unwrap();
//! unsafe {
//!    let text = CStr::from_ptr(lv_label_get_text(label_widget.raw()));
//!    assert_eq!(text, c"Example label");
//! }
//! ```
//!
//! ```
//! # use lv_bevy_ecs::widgets::{Widget, Label, LvglWorld};
//! # use lv_bevy_ecs::functions::*;
//! # use lv_bevy_ecs::bevy::prelude::*;
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! # let mut world = LvglWorld::default();
//! # let mut label: Widget = Label::create_widget();
//! # lv_label_set_text(&mut label, c"Example label 1");
//! # let mut label_entity = world.spawn((Label, label));
//! #
//! let mut labels = world.query_filtered::<&mut Widget, With<Label>>();
//! assert_eq!(labels.iter(&world).count(),1);
//! ```
//!
//! In case of a unique entity:
//! ```
//! # use lv_bevy_ecs::widgets::{Widget, Label, LvglWorld};
//! # use lv_bevy_ecs::functions::*;
//! # use lv_bevy_ecs::bevy::prelude::*;
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! # let mut world = LvglWorld::default();
//! # let mut label: Widget = Label::create_widget();
//! # lv_label_set_text(&mut label, c"Example label");
//! # let mut label_entity = world.spawn((Label, label));
//! #
//! let mut label = world.query_filtered::<&mut Widget, With<Label>>().single_mut(&mut world).unwrap();
//! ```
//!
//! You are free to define any kind of custom component:
//!
//! ```
//! # use lv_bevy_ecs::widgets::{Widget, Label, LvglWorld};
//! # use lv_bevy_ecs::functions::*;
//! # use lv_bevy_ecs::bevy::prelude::*;
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! #[derive(Component)]
//! struct DynamicLabel;
//!
//! # let mut world = LvglWorld::default();
//! # let mut label: Widget = Label::create_widget();
//! # lv_label_set_text(&mut label, c"Example label");
//! #
//! world.spawn((Label, label, DynamicLabel));
//! //...
//! let mut label = world.query_filtered::<&mut Widget, With<DynamicLabel>>().single_mut(&mut world).unwrap();
//! ```
//!
//! ## Child widgets
//! To add a widget as a child, set it as child entity
//! ```
//! # use lv_bevy_ecs::widgets::{Widget, Label, LvglWorld, Button};
//! # use lv_bevy_ecs::functions::*;
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! # let mut world = LvglWorld::default();
//! let mut button: Widget = Button::create_widget();
//! let mut label: Widget = Label::create_widget();
//! lv_label_set_text(&mut label, c"Example label");
//!
//! let mut button_entity = world.spawn((Button, button));
//! let mut label_entity = button_entity.with_child((Label, label));
//!
//! let mut button_widget = button_entity.get::<Widget>().unwrap();
//! assert_eq!(lv_obj_get_child_count(button_widget), 1)
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
use lightvgl_sys::lv_obj_t;

pub struct LvglWorld(World);

impl Default for LvglWorld {
    fn default() -> Self {
        let mut world = World::new();
        world.add_observer(on_insert_parent);
        Self(world)
    }
}

impl Deref for LvglWorld {
    type Target = World;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LvglWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, PartialEq)]
pub struct Widget {
    raw: NonNull<lv_obj_t>,
}

impl Widget {
    pub fn raw(&self) -> *const lv_obj_t {
        self.raw.as_ptr()
    }

    pub fn raw_mut(&mut self) -> *mut lv_obj_t {
        self.raw.as_ptr()
    }

    pub fn from_ptr(ptr: *mut lv_obj_t) -> Option<Self> {
        Some(Self {
            raw: NonNull::new(ptr)?,
        })
    }

    pub fn from_non_null(ptr: NonNull<lv_obj_t>) -> Self {
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
            // Small delay is needed to prevent double-freeing child objects
            // TODO more safe solution
            lightvgl_sys::lv_obj_delete_async(self.raw.as_ptr());
        }
    }
}

macro_rules! impl_widget {
    ($t:ident, $func:path) => {
        #[derive(bevy_ecs::component::Component)]
        #[component(storage = "SparseSet")]
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

#[allow(clippy::type_complexity)]
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
    /// Convert LVGL Obj pointer to Wdg or panic if null pointer was given
    pub fn from_ptr(ptr: *mut lv_obj_t) -> Self {
        Self {
            raw: NonNull::new(ptr).unwrap(),
        }
    }

    /// Convert LVGL Obj pointer to Some(Wdg) or None if null pointer was given
    pub fn try_from_ptr(ptr: *mut lv_obj_t) -> Option<Self> {
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

    pub fn from_non_null(ptr: &NonNull<lv_obj_t>) -> &Self {
        unsafe { &*(ptr as *const _ as *const Self) }
    }

    pub fn from_non_null_mut(ptr: &mut NonNull<lv_obj_t>) -> &mut Self {
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
