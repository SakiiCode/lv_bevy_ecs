//! # Widgets
//!
//! The most basic type is the `Widget` struct that represents an owned `lv_obj_t`. If it goes out of scope,
//! the widget will be deleted. Its borrowed form is the `Wdg` struct, it does not delete the widget upon drop.
//!
//! `Widget` and `Wdg` can be mutable or immutable, exclusive access is not enforced. If you happen to have a
//! `*mut lv_obj_t` you can turn it into a `Wdg` with `Wdg::from_ptr(*mut lv_obj_t)`. Alternatively, `Widget::from_ptr(*mut lv_obj_t)`
//! can be used, but that will destroy the widget if goes out of scope.
//!
//! ## Using widgets
//! ```
//! # use lv_bevy_ecs::widgets::*;
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! let mut world: LvglWorld = LvglWorld::default();
//! let mut label: Label<Widget> = Label::new();
//! label.set_text(c"Example label");
//! let mut label_entity = world.spawn(label.into_inner());
//! ```
//!
//! To create widgets, you have to use the specific struct (Arc, Label, Button, ...) `new()` function that
//! create `Something<Widget>` objects. These structs can be generic over `<Widget>` or `<Wdg>` depending on
//! whether they are owned or borrowed.
//!
//! To convert a `Widget` or `Wdg` back to a specific type, the `.downcast()` or `.downcast_mut()` method can be used.
//!
//! #### Widget functions
//!
//! You can access both `lv_obj_some_function` and `lv_widgettype_other_function` using `yourwidget.some_function(params)`
//! and `yourwidget.other_function(params)` respectively.
//!
//! They are usually attached to `&mut self` but `&self` is also common.
//!
//! #### Storing widgets
//!
//! As explained in the readme, widgets should be moved to a storage system so that they will be accessible from elsewhere and don't get deallocated.
//!
//! In case of `bevy_ecs`, this is done using the `LvglWorld.spawn(Widget)` function. Since you usually have a generic `Something<Widget>`, which
//! does not implement `Component`, it needs to be converted back to a `Widget` using the `.to_inner()` function.
//!
//! #### Modifying Widgets
//!
//! To access widgets after moving them to the World with the `spawn()` function, you have to store the created Entity ID or use queries.
//!
//! ```
//! # use core::ffi::CStr;
//! # use lv_bevy_ecs::widgets::{Widget, Label, LvglWorld, Wdg};
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! # let mut world = LvglWorld::default();
//! # let mut label = Label::new();
//! # label.set_text(c"Example label");
//! # let mut label_entity = world.spawn(label.into_inner());
//! #
//! let label_widget = label_entity.get::<Widget>().unwrap();
//! let label: &Label<Wdg> = label_widget.downcast().unwrap();
//!
//! let text = label.get_text();
//! assert_eq!(text, c"Example label");
//! ```
//!
//! ```
//! # use lv_bevy_ecs::widgets::{Widget, Label, LvglWorld};
//! # use lv_bevy_ecs::bevy::prelude::*;
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! # let mut world = LvglWorld::default();
//! # let mut label = Label::new();
//! # let mut label_entity = world.spawn(label.into_inner());
//! #
//! let mut widgets = world.query::<&mut Widget>();
//! assert_eq!(widgets.iter(&world).count(),1);
//! ```
//!
//! You are free to define any kind of custom component for easier lookup:
//!
//! ```
//! # use lv_bevy_ecs::widgets::{Widget, Label, LvglWorld};
//! # use lv_bevy_ecs::bevy::prelude::*;
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! #[derive(Component)]
//! struct DynamicLabel;
//!
//! # let mut world = LvglWorld::default();
//! # let mut label = Label::new();
//! #
//! world.spawn((label.into_inner(), DynamicLabel));
//! //...
//! let mut label = world
//!     .query_filtered::<&mut Widget, With<DynamicLabel>>()
//!     .single_mut(&mut world)
//!     .unwrap();
//! ```
//!
//! #### Child widgets
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
//! label.set_text(c"Example label");
//!
//! let mut button_entity = world.spawn(button.into_inner());
//! let mut label_entity = button_entity.with_child(label.into_inner());
//!
//! let mut button_widget = button_entity.get::<Widget>().unwrap();
//! assert_eq!(button_widget.get_child_count(), 1);
//! ```

use ::core::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};
use core::{ffi::CStr, mem::MaybeUninit};

use alloc::string::{String, ToString};
use bevy_ecs::{
    component::Component,
    hierarchy::ChildOf,
    lifecycle::Insert,
    observer::On,
    system::{ParamSet, Query},
    world::World,
};
#[cfg(feature = "no_ecs")]
use lightvgl_sys::lv_style_selector_t;
use lightvgl_sys::{lv_label_create, lv_obj_class_t, lv_obj_get_class, lv_obj_t};
use thiserror::Error;

#[cfg(feature = "no_ecs")]
use crate::styles::Style;
use crate::{
    events::{Event, EventCode},
    info,
};

/// An [LvglWorld] wrapper that is `const` compatible, but must be initalized manually before first use using `.init()`
///
/// It never checks whether the underlying data has been initialized, potentially causing undefined behaviour
///
/// If unsure, just use `std::sync::LazyLock` *(std)* or `once_cell::sync::Lazy` *(no_std)*
#[repr(transparent)]
pub struct UnsafeLvglWorld(MaybeUninit<LvglWorld>);

impl UnsafeLvglWorld {
    pub const fn new() -> Self {
        Self(MaybeUninit::uninit())
    }
    pub fn init(&mut self) {
        self.0.write(LvglWorld::default());
    }
}

impl Default for UnsafeLvglWorld {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for UnsafeLvglWorld {
    type Target = LvglWorld;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.assume_init_ref() }
    }
}

impl DerefMut for UnsafeLvglWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.assume_init_mut() }
    }
}

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

impl LvglWorld {
    #[inline]
    pub fn new() -> Self {
        Self::default()
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
            // Async is needed to prevent double-freeing child objects
            lightvgl_sys::lv_obj_delete_async(self.raw.as_ptr());
        }
    }
}

pub type Obj = Widget;

#[derive(Debug, Error)]
#[non_exhaustive]
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

    pub fn downcast<T: WidgetSpec>(&self) -> Result<&T, DowncastError> {
        T::from_non_null(&self.raw)
    }

    pub fn downcast_mut<T: WidgetSpec>(&mut self) -> Result<&mut T, DowncastError> {
        T::from_non_null_mut(&mut self.raw)
    }

    pub fn add_event_cb<'a, F>(&mut self, filter: EventCode, callback: F)
    where
        F: FnMut(Event),
    {
        crate::events::lv_obj_add_event_cb(self, filter, callback)
    }

    #[cfg(feature = "no_ecs")]
    /// ## Safety
    /// You need to make sure the given Style does not get deallocated, otherwise this will cause a
    /// use-after-free.
    ///
    /// For example `Box::leak(Box::new(style))` can be used to prevent dropping it.
    pub unsafe fn add_style(&mut self, style: &mut Style, selector: lv_style_selector_t) {
        unsafe { lightvgl_sys::lv_obj_add_style(self.raw_mut(), style.raw_mut(), selector) }
    }

    #[cfg(feature = "no_ecs")]
    pub fn set_parent(&mut self, parent: &mut Wdg) {
        unsafe { lightvgl_sys::lv_obj_set_parent(self.raw_mut(), parent.raw_mut()) }
    }
}

fn check_class(obj: *const lv_obj_t, other_class: &lv_obj_class_t) -> Result<(), DowncastError> {
    unsafe {
        if !lightvgl_sys::lv_obj_check_type(obj, other_class) {
            let current_cstr = CStr::from_ptr((*lv_obj_get_class(obj)).name);
            let current_string = current_cstr.to_string_lossy();
            let expected_cstr = CStr::from_ptr(other_class.name);
            let expected_string = expected_cstr.to_string_lossy();
            Err(DowncastError::NotMatching {
                actual: current_string.to_string(),
                expected: expected_string.to_string(),
            })
        } else {
            Ok(())
        }
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

impl WidgetSpec for SimpleObject<Wdg> {
    fn get_class() -> &'static lv_obj_class_t {
        unsafe { &lightvgl_sys::lv_label_class }
    }

    fn from_non_null(ptr: &NonNull<lv_obj_t>) -> Result<&Self, DowncastError> {
        check_class(ptr.as_ptr(), Self::get_class())?;
        Ok(unsafe { &*(ptr as *const _ as *const Self) })
    }

    fn from_non_null_mut(ptr: &mut NonNull<lv_obj_t>) -> Result<&mut Self, DowncastError> {
        check_class(ptr.as_ptr(), Self::get_class())?;
        Ok(unsafe { &mut *(ptr as *mut _ as *mut Self) })
    }
}

impl Deref for SimpleObject<Widget> {
    type Target = SimpleObject<Wdg>;
    fn deref(&self) -> &Self::Target {
        SimpleObject::<Wdg>::from_non_null(&self.0.raw).unwrap()
    }
}

impl DerefMut for SimpleObject<Widget> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        SimpleObject::<Wdg>::from_non_null_mut(&mut self.0.raw).unwrap()
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

pub trait WidgetSpec {
    fn get_class() -> &'static lv_obj_class_t;

    fn from_non_null(ptr: &NonNull<lv_obj_t>) -> Result<&Self, DowncastError>;

    fn from_non_null_mut(ptr: &mut NonNull<lv_obj_t>) -> Result<&mut Self, DowncastError>;
}

macro_rules! impl_widget {
    ($t:ident, $func:path, $class:path) => {
        pub struct $t<T: RawObj>(T);

        impl<T: RawObj> WidgetSpec for $t<T> {
            fn get_class() -> &'static lv_obj_class_t {
                unsafe { &$class }
            }

            fn from_non_null(ptr: &NonNull<lv_obj_t>) -> Result<&Self, DowncastError> {
                check_class(ptr.as_ptr(), Self::get_class())?;
                Ok(unsafe { &*(ptr as *const _ as *const Self) })
            }

            fn from_non_null_mut(ptr: &mut NonNull<lv_obj_t>) -> Result<&mut Self, DowncastError> {
                check_class(ptr.as_ptr(), Self::get_class())?;
                Ok(unsafe { &mut *(ptr as *mut _ as *mut Self) })
            }
        }

        impl<T: RawObj> $t<T> {
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
                    let current_screen = lightvgl_sys::lv_screen_active();
                    let ptr = $func(current_screen);
                    Some(Self(crate::widgets::Widget::from_ptr(ptr)?))
                }
            }

            pub fn leak(self) -> Wdg {
                self.0.leak()
            }
        }

        impl Deref for $t<Widget> {
            type Target = $t<Wdg>;
            fn deref(&self) -> &Self::Target {
                $t::from_non_null(&self.0.raw).unwrap()
            }
        }

        impl DerefMut for $t<Widget> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                $t::from_non_null_mut(&mut self.0.raw).unwrap()
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
    };
}

include!(concat!(env!("OUT_DIR"), "/widgets.rs"));
