//! # Events
//!
//! Events don't yet have that `.shorthand()` methods that widgets have.
//! The original LVGL function syntax need to be used.
//! The `lv_bevy_ecs::functions::*` module contains some safe wrappers.
//!
use core::{
    ffi::c_void,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use ::alloc::boxed::Box;
use lightvgl_sys::{lv_event_code_t, lv_event_t};

use crate::widgets::Wdg;

/// Events are triggered in LVGL when something happens which might be interesting to
/// the user, e.g. if an object:
///  - is clicked
///  - is dragged
///  - its value has changed, etc.
///
/// All objects (such as Buttons/Labels/Sliders etc.) receive these generic events
/// regardless of their type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[allow(clippy::empty_docs)]
pub enum EventCode {
    /// The object has been pressed
    Pressed,

    /// The object is being pressed (sent continuously while pressing)
    Pressing,

    /// The input device is still being pressed but is no longer on the object
    PressLost,

    /// Released before `long_press_time` config time. Not called if dragged.
    ShortClicked,

    /// Called on release if not dragged (regardless to long press)
    Clicked,

    /// Pressing for `long_press_time` config time. Not called if dragged.
    LongPressed,

    /// Called after `long_press_time` config in every `long_press_rep_time` ms. Not
    /// called if dragged.
    LongPressedRepeat,

    /// Called in every case when the object has been released even if it was dragged. Not called
    /// if slid from the object while pressing and released outside of the object. In this
    /// case, `Event<_>::PressLost` is sent.
    Released,

    /// Called when an underlying value is changed e.g. position of a `Slider`.
    ValueChanged,

    ///
    DrawMain,

    ///
    DrawMainBegin,

    ///
    DrawMainEnd,

    ///
    DrawPost,

    ///
    DrawPostBegin,

    ///
    DrawPostEnd,

    /// Called on focus
    Focused,
}

impl TryFrom<lv_event_code_t> for EventCode {
    type Error = ();

    fn try_from(value: lv_event_code_t) -> Result<Self, Self::Error> {
        const LV_EVENT_PRESSED: lv_event_code_t = lightvgl_sys::lv_event_code_t_LV_EVENT_PRESSED;
        const LV_EVENT_PRESSING: lv_event_code_t = lightvgl_sys::lv_event_code_t_LV_EVENT_PRESSING;
        const LV_EVENT_PRESS_LOST: lv_event_code_t =
            lightvgl_sys::lv_event_code_t_LV_EVENT_PRESS_LOST;
        const LV_EVENT_SHORT_CLICKED: lv_event_code_t =
            lightvgl_sys::lv_event_code_t_LV_EVENT_SHORT_CLICKED;
        const LV_EVENT_CLICKED: lv_event_code_t = lightvgl_sys::lv_event_code_t_LV_EVENT_CLICKED;
        const LV_EVENT_LONG_PRESSED: lv_event_code_t =
            lightvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED;
        const LV_EVENT_LONG_PRESSED_REPEAT: lv_event_code_t =
            lightvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED_REPEAT;
        const LV_EVENT_RELEASED: lv_event_code_t = lightvgl_sys::lv_event_code_t_LV_EVENT_RELEASED;
        const LV_EVENT_VALUE_CHANGED: lv_event_code_t =
            lightvgl_sys::lv_event_code_t_LV_EVENT_VALUE_CHANGED;
        const LV_EVENT_DRAW_MAIN: lv_event_code_t =
            lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN;
        const LV_EVENT_DRAW_MAIN_BEGIN: lv_event_code_t =
            lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_BEGIN;
        const LV_EVENT_DRAW_MAIN_END: lv_event_code_t =
            lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_END;
        const LV_EVENT_DRAW_POST: lv_event_code_t =
            lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST;
        const LV_EVENT_DRAW_POST_BEGIN: lv_event_code_t =
            lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_BEGIN;
        const LV_EVENT_DRAW_POST_END: lv_event_code_t =
            lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_END;

        match value {
            LV_EVENT_PRESSED => Ok(EventCode::Pressed),
            LV_EVENT_PRESSING => Ok(EventCode::Pressing),
            LV_EVENT_PRESS_LOST => Ok(EventCode::PressLost),
            LV_EVENT_SHORT_CLICKED => Ok(EventCode::ShortClicked),
            LV_EVENT_CLICKED => Ok(EventCode::Clicked),
            LV_EVENT_LONG_PRESSED => Ok(EventCode::LongPressed),
            LV_EVENT_LONG_PRESSED_REPEAT => Ok(EventCode::LongPressedRepeat),
            LV_EVENT_RELEASED => Ok(EventCode::Released),
            LV_EVENT_VALUE_CHANGED => Ok(EventCode::ValueChanged),
            LV_EVENT_DRAW_MAIN => Ok(EventCode::DrawMain),
            LV_EVENT_DRAW_MAIN_BEGIN => Ok(EventCode::DrawMainBegin),
            LV_EVENT_DRAW_MAIN_END => Ok(EventCode::DrawMainEnd),
            LV_EVENT_DRAW_POST => Ok(EventCode::DrawPost),
            LV_EVENT_DRAW_POST_BEGIN => Ok(EventCode::DrawPostBegin),
            LV_EVENT_DRAW_POST_END => Ok(EventCode::DrawPostEnd),
            _ => Err(()),
        }
    }
}

impl From<EventCode> for lightvgl_sys::lv_event_code_t {
    fn from(event: EventCode) -> Self {
        let native_event = match event {
            EventCode::Pressed => lightvgl_sys::lv_event_code_t_LV_EVENT_PRESSED,
            EventCode::Pressing => lightvgl_sys::lv_event_code_t_LV_EVENT_PRESSING,
            EventCode::PressLost => lightvgl_sys::lv_event_code_t_LV_EVENT_PRESS_LOST,
            EventCode::ShortClicked => lightvgl_sys::lv_event_code_t_LV_EVENT_SHORT_CLICKED,
            EventCode::Clicked => lightvgl_sys::lv_event_code_t_LV_EVENT_CLICKED,
            EventCode::LongPressed => lightvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED,
            EventCode::LongPressedRepeat => {
                lightvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED_REPEAT
            }
            EventCode::Released => lightvgl_sys::lv_event_code_t_LV_EVENT_RELEASED,
            EventCode::ValueChanged => lightvgl_sys::lv_event_code_t_LV_EVENT_VALUE_CHANGED,
            EventCode::DrawMain => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN,
            EventCode::DrawMainBegin => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_BEGIN,
            EventCode::DrawMainEnd => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_END,
            EventCode::DrawPost => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST,
            EventCode::DrawPostBegin => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_BEGIN,
            EventCode::DrawPostEnd => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_END,
            // TODO: handle all types...
            _ => lightvgl_sys::lv_event_code_t_LV_EVENT_CLICKED,
        };
        native_event as lightvgl_sys::lv_event_code_t
    }
}

pub(crate) fn lv_obj_add_event_cb<F>(widget: &mut Wdg, filter: EventCode, callback: F)
where
    F: FnMut(Event),
{
    unsafe {
        lightvgl_sys::lv_obj_add_event_cb(
            widget.raw_mut(),
            Some(event_callback::<F>),
            filter.into(),
            Box::into_raw(Box::new(callback)).cast(),
        );
    }
}

pub(crate) unsafe extern "C" fn event_callback<F>(event: *mut lightvgl_sys::lv_event_t)
where
    F: FnMut(Event),
{
    unsafe {
        let user_data = lightvgl_sys::lv_event_get_user_data(event);
        let callback = &mut *(user_data.cast::<F>());
        let event_ref = Event::from_ptr(event);
        callback(event_ref);
    }
}

pub struct Event {
    raw: NonNull<lv_event_t>,
}

impl Deref for Event {
    type Target = lv_event_t;
    fn deref(&self) -> &Self::Target {
        unsafe { self.raw.as_ref() }
    }
}

impl DerefMut for Event {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.raw.as_mut() }
    }
}

impl Event {
    pub fn raw(&self) -> *const lv_event_t {
        self.raw.as_ptr().cast_const()
    }

    pub fn raw_mut(&mut self) -> *mut lv_event_t {
        self.raw.as_ptr()
    }

    pub fn from_ptr(ptr: *mut lv_event_t) -> Self {
        Self {
            raw: NonNull::new(ptr).unwrap(),
        }
    }

    // TODO: make lv_event... functions autogenerated
    pub fn get_target(&mut self) -> *const c_void {
        unsafe { lightvgl_sys::lv_event_get_target(self.raw_mut()) }
    }

    pub fn get_target_obj(&mut self) -> Option<Wdg> {
        unsafe {
            let target = lightvgl_sys::lv_event_get_target_obj(self.raw_mut());
            Wdg::try_from_ptr(target)
        }
    }

    pub fn get_current_target_obj(&mut self) -> Option<Wdg> {
        unsafe {
            let target = lightvgl_sys::lv_event_get_current_target_obj(self.raw_mut());
            Wdg::try_from_ptr(target)
        }
    }
}
