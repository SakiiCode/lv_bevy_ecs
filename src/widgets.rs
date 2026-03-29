//! # Widgets
//!
//! ```
//! # use lv_bevy_ecs::widgets::*;
//! # use lv_bevy_ecs::functions::*;
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! let mut world = LvglWorld::default();
//! let mut label = Label::new();
//! lv_label_set_text(&mut label, c"Example label");
//! let mut label_entity = world.spawn(label.into_inner());
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
//! # let mut label = Label::new();
//! # lv_label_set_text(&mut label, c"Example label");
//! # let mut label_entity = world.spawn(label.into_inner());
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
//! # let mut label = Label::new();
//! # lv_label_set_text(&mut label, c"Example label 1");
//! # let mut label_entity = world.spawn(label.into_inner());
//! #
//! let mut widgets = world.query::<&mut Widget>();
//! assert_eq!(widgets.iter(&world).count(),1);
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
//! # let mut label = Label::new();
//! # lv_label_set_text(&mut label, c"Example label");
//! #
//! world.spawn((label.into_inner(), DynamicLabel));
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
//! let mut button = Button::new();
//! let mut label = Label::new();
//! lv_label_set_text(&mut label, c"Example label");
//!
//! let mut button_entity = world.spawn(button.into_inner());
//! let mut label_entity = button_entity.with_child(label.into_inner());
//!
//! let mut button_widget = button_entity.get::<Widget>().unwrap();
//! assert_eq!(lv_obj_get_child_count(button_widget), 1)
//! ```

use ::core::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};
use core::ffi::CStr;

use alloc::string::{String, ToString};
use bevy_ecs::{
    component::Component,
    hierarchy::ChildOf,
    lifecycle::Insert,
    observer::On,
    system::{ParamSet, Query},
    world::World,
};
use lightvgl_sys::{lv_label_create, lv_obj_t};
use thiserror::Error;

use crate::info;

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

    pub fn leak(self) -> Wdg {
        let wdg = Wdg::from_ptr(self.raw.as_ptr());
        ::core::mem::forget(self);
        wdg
    }

    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Widget {
    fn default() -> Self {
        unsafe {
            Widget::from_ptr(lightvgl_sys::lv_obj_create(lightvgl_sys::lv_screen_active())).unwrap()
        }
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

pub type Obj = Widget;

#[derive(Debug, Error)]
pub enum DowncastError {
    #[error("lv_obj_class not compatible (actual:{actual}, expected:{expected})")]
    NotMatching { actual: String, expected: String },
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

pub trait RawObj {
    fn raw(&self) -> *const lv_obj_t;
    fn raw_mut(&mut self) -> *mut lv_obj_t;
}

impl RawObj for Widget {
    fn raw(&self) -> *const lv_obj_t {
        self.raw.as_ptr().cast_const()
    }
    fn raw_mut(&mut self) -> *mut lv_obj_t {
        self.raw.as_ptr()
    }
}

impl RawObj for Wdg {
    fn raw(&self) -> *const lv_obj_t {
        self.raw.as_ptr().cast_const()
    }
    fn raw_mut(&mut self) -> *mut lv_obj_t {
        self.raw.as_ptr()
    }
}

pub struct SimpleObject<T: RawObj>(T);

impl<T: RawObj> SimpleObject<T> {
    pub fn raw(&self) -> *const lv_obj_t {
        self.0.raw()
    }

    pub fn raw_mut(&mut self) -> *mut lv_obj_t {
        self.0.raw_mut()
    }

    pub fn set_text(&mut self, text: &CStr) {
        unsafe {
            lightvgl_sys::lv_label_set_text(self.raw_mut(), text.as_ptr());
        }
    }
}

impl SimpleObject<Widget> {
    pub fn new() -> Self {
        unsafe { Self(Widget::from_ptr(lv_label_create(core::ptr::null_mut())).unwrap()) }
    }
}

impl SimpleObject<Wdg> {
    fn from_non_null(ptr: &NonNull<lv_obj_t>) -> &Self {
        unsafe { &*(ptr as *const _ as *const Self) }
    }

    fn from_non_null_mut(ptr: &mut NonNull<lv_obj_t>) -> &mut Self {
        unsafe { &mut *(ptr as *mut _ as *mut Self) }
    }
}

impl Deref for SimpleObject<Widget> {
    type Target = SimpleObject<Wdg>;
    fn deref(&self) -> &Self::Target {
        SimpleObject::<Wdg>::from_non_null(&self.0.raw)
    }
}

impl DerefMut for SimpleObject<Widget> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        SimpleObject::<Wdg>::from_non_null_mut(&mut self.0.raw)
    }
}

impl Deref for SimpleObject<Wdg> {
    type Target = Wdg;
    fn deref(&self) -> &Self::Target {
        Wdg::from_non_null(&self.0.raw)
    }
}

impl DerefMut for SimpleObject<Wdg> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Wdg::from_non_null_mut(&mut self.0.raw)
    }
}

impl From<SimpleObject<Widget>> for Widget {
    fn from(value: SimpleObject<Widget>) -> Self {
        value.0
    }
}

impl From<SimpleObject<Wdg>> for Wdg {
    fn from(value: SimpleObject<Wdg>) -> Self {
        value.0
    }
}

/*impl TryFrom<Widget> for SimpleObject<Widget> {
    type Error = DowncastError;
    fn try_from(value: Widget) -> Result<Self, Self::Error> {
        unsafe {
            if !value.check_type(&lightvgl_sys::lv_label_class) {
                let current_cstr = CStr::from_ptr(value.get_class().unwrap().as_ref().name);
                let current_string = current_cstr.to_string_lossy();
                let expected_cstr = CStr::from_ptr(lightvgl_sys::lv_label_class.name);
                let expected_string = expected_cstr.to_string_lossy();
                return Err(DowncastError::NotMatching {
                    actual: current_string.to_string(),
                    expected: expected_string.to_string(),
                });
            }
        }
        return Ok(SimpleObject(value));
    }
}

impl<'a> TryFrom<&'a mut Widget> for &'a SimpleObject<Wdg> {
    type Error = DowncastError;
    fn try_from(value: &'a mut Widget) -> Result<Self, Self::Error> {
        unsafe {
            if !value.check_type(&lightvgl_sys::lv_label_class) {
                let current_cstr = CStr::from_ptr(value.get_class().unwrap().as_ref().name);
                let current_string = current_cstr.to_string_lossy();
                let expected_cstr = CStr::from_ptr(lightvgl_sys::lv_label_class.name);
                let expected_string = expected_cstr.to_string_lossy();
                return Err(DowncastError::NotMatching {
                    actual: current_string.to_string(),
                    expected: expected_string.to_string(),
                });
            }
        }
        return Ok(SimpleObject::from_non_null(&value.raw));
    }
}

impl<'a> TryFrom<&'a Wdg> for &'a SimpleObject<Wdg> {
    type Error = DowncastError;
    fn try_from(value: &'a Wdg) -> Result<Self, Self::Error> {
        unsafe {
            if !value.check_type(&lightvgl_sys::lv_label_class) {
                let current_cstr = CStr::from_ptr(value.get_class().unwrap().as_ref().name);
                let current_string = current_cstr.to_string_lossy();
                let expected_cstr = CStr::from_ptr(lightvgl_sys::lv_label_class.name);
                let expected_string = expected_cstr.to_string_lossy();
                return Err(DowncastError::NotMatching {
                    actual: current_string.to_string(),
                    expected: expected_string.to_string(),
                });
            }
        }
        return Ok(SimpleObject::from_non_null(&value.raw));
    }
}

impl TryFrom<Wdg> for SimpleObject<Wdg> {
    type Error = DowncastError;
    fn try_from(value: Wdg) -> Result<Self, Self::Error> {
        unsafe {
            if !value.check_type(&lightvgl_sys::lv_label_class) {
                let current_cstr = CStr::from_ptr(value.get_class().unwrap().as_ref().name);
                let current_string = current_cstr.to_string_lossy();
                let expected_cstr = CStr::from_ptr(lightvgl_sys::lv_label_class.name);
                let expected_string = expected_cstr.to_string_lossy();
                return Err(DowncastError::NotMatching {
                    actual: current_string.to_string(),
                    expected: expected_string.to_string(),
                });
            }
        }
        return Ok(SimpleObject(value));
    }
}*/
fn asd() {
    let mut a = SimpleObject::new();
    a.set_text(c"asdsdad");
    let mut world = World::new();
    //lv_label_set_text(&mut *a, c"asdsad");
    a.universal_func();
    //let widget: Widget = a.into();
    //world.spawn(a);
    //let lbl: SimpleObject<Widget> = widget.try_into().unwrap();
    //let lbl_borrow: SimpleObject<Wdg> = asdasd.try_into().unwrap();
}

impl Wdg {
    fn universal_func(&self) {}
}

macro_rules! impl_widget {
    ($t:ident, $func:path, $class:path) => {
        pub struct $t<T: RawObj>(T);

        impl<T: RawObj> $t<T> {
            pub fn raw(&self) -> *const lv_obj_t {
                self.0.raw()
            }

            pub fn raw_mut(&mut self) -> *mut lv_obj_t {
                self.0.raw_mut()
            }

            pub fn into_inner(self) -> T {
                self.0
            }
        }

        impl $t<Widget> {
            pub fn new() -> Self {
                Self::try_new().expect("Could not create widget")
            }

            pub fn try_new() -> Option<Self> {
                unsafe {
                    let default_screen = lightvgl_sys::lv_display_get_screen_active(
                        lightvgl_sys::lv_display_get_default(),
                    );
                    let ptr = $func(default_screen);
                    Some(Self(crate::widgets::Widget::from_ptr(ptr)?))
                }
            }

            pub fn leak(self) -> Wdg {
                self.0.leak()
            }
        }

        // TODO check for class
        impl $t<Wdg> {
            fn from_non_null(ptr: &NonNull<lv_obj_t>) -> &Self {
                unsafe { &*(ptr as *const _ as *const Self) }
            }

            fn from_non_null_mut(ptr: &mut NonNull<lv_obj_t>) -> &mut Self {
                unsafe { &mut *(ptr as *mut _ as *mut Self) }
            }
        }

        impl Deref for $t<Widget> {
            type Target = $t<Wdg>;
            fn deref(&self) -> &Self::Target {
                $t::from_non_null(&self.0.raw)
            }
        }

        impl DerefMut for $t<Widget> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                $t::from_non_null_mut(&mut self.0.raw)
            }
        }

        impl Deref for $t<Wdg> {
            type Target = Wdg;
            fn deref(&self) -> &Self::Target {
                Wdg::from_non_null(&self.0.raw)
            }
        }

        impl DerefMut for $t<Wdg> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                Wdg::from_non_null_mut(&mut self.0.raw)
            }
        }

        impl<T: RawObj> AsRef<T> for $t<T> {
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<T: RawObj> AsMut<T> for $t<T> {
            fn as_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }

        impl From<$t<Widget>> for Widget {
            fn from(value: $t<Widget>) -> Self {
                value.0
            }
        }

        impl From<$t<Wdg>> for Wdg {
            fn from(value: $t<Wdg>) -> Self {
                value.0
            }
        }

        impl TryFrom<Widget> for $t<Widget> {
            type Error = DowncastError;
            fn try_from(value: Widget) -> Result<Self, Self::Error> {
                unsafe {
                    if !value.check_type(&$class) {
                        let current_cstr = CStr::from_ptr(value.get_class().unwrap().as_ref().name);
                        let current_string = current_cstr.to_string_lossy();
                        let expected_cstr = CStr::from_ptr($class.name);
                        let expected_string = expected_cstr.to_string_lossy();
                        return Err(DowncastError::NotMatching {
                            actual: current_string.to_string(),
                            expected: expected_string.to_string(),
                        });
                    }
                }
                return Ok($t(value));
            }
        }

        impl<'a> TryFrom<&'a mut Widget> for &'a mut $t<Wdg> {
            type Error = DowncastError;
            fn try_from(value: &'a mut Widget) -> Result<Self, Self::Error> {
                unsafe {
                    if !value.check_type(&$class) {
                        let current_cstr = CStr::from_ptr(value.get_class().unwrap().as_ref().name);
                        let current_string = current_cstr.to_string_lossy();
                        let expected_cstr = CStr::from_ptr($class.name);
                        let expected_string = expected_cstr.to_string_lossy();
                        return Err(DowncastError::NotMatching {
                            actual: current_string.to_string(),
                            expected: expected_string.to_string(),
                        });
                    }
                }
                return Ok($t::from_non_null_mut(&mut value.raw));
            }
        }

        impl<'a> TryFrom<&'a Wdg> for &'a $t<Wdg> {
            type Error = DowncastError;
            fn try_from(value: &'a Wdg) -> Result<Self, Self::Error> {
                unsafe {
                    if !value.check_type(&$class) {
                        let current_cstr = CStr::from_ptr(value.get_class().unwrap().as_ref().name);
                        let current_string = current_cstr.to_string_lossy();
                        let expected_cstr = CStr::from_ptr($class.name);
                        let expected_string = expected_cstr.to_string_lossy();
                        return Err(DowncastError::NotMatching {
                            actual: current_string.to_string(),
                            expected: expected_string.to_string(),
                        });
                    }
                }
                return Ok($t::from_non_null(&value.raw));
            }
        }

        impl<'a> TryFrom<&'a mut Wdg> for &'a mut $t<Wdg> {
            type Error = DowncastError;
            fn try_from(value: &'a mut Wdg) -> Result<Self, Self::Error> {
                unsafe {
                    if !value.check_type(&$class) {
                        let current_cstr = CStr::from_ptr(value.get_class().unwrap().as_ref().name);
                        let current_string = current_cstr.to_string_lossy();
                        let expected_cstr = CStr::from_ptr($class.name);
                        let expected_string = expected_cstr.to_string_lossy();
                        return Err(DowncastError::NotMatching {
                            actual: current_string.to_string(),
                            expected: expected_string.to_string(),
                        });
                    }
                }
                return Ok($t::from_non_null_mut(&mut value.raw));
            }
        }

        impl TryFrom<Wdg> for $t<Wdg> {
            type Error = DowncastError;
            fn try_from(value: Wdg) -> Result<Self, Self::Error> {
                unsafe {
                    if !value.check_type(&$class) {
                        let current_cstr = CStr::from_ptr(value.get_class().unwrap().as_ref().name);
                        let current_string = current_cstr.to_string_lossy();
                        let expected_cstr = CStr::from_ptr($class.name);
                        let expected_string = expected_cstr.to_string_lossy();
                        return Err(DowncastError::NotMatching {
                            actual: current_string.to_string(),
                            expected: expected_string.to_string(),
                        });
                    }
                }
                return Ok($t(value));
            }
        }
    };
}

include!(concat!(env!("OUT_DIR"), "/widgets.rs"));
