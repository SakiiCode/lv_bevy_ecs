//! # Events
//!
//!
use core::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use ::alloc::boxed::Box;
use lightvgl_sys::{lv_event_code_t, lv_event_t};

use crate::widgets::{RawObj, Wdg};

pub(crate) fn lv_obj_add_event_cb<F>(widget: &mut Wdg, filter: lv_event_code_t, callback: F)
where
    F: FnMut(Event) + 'static,
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
    F: FnMut(Event) + 'static,
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
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.raw.as_ref() }
    }
}

impl DerefMut for Event {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.raw.as_mut() }
    }
}

impl Event {
    #[inline]
    pub fn raw(&self) -> *const lv_event_t {
        self.raw.as_ptr().cast_const()
    }

    #[inline]
    pub fn raw_mut(&mut self) -> *mut lv_event_t {
        self.raw.as_ptr()
    }

    #[inline]
    pub fn from_ptr(ptr: *mut lv_event_t) -> Self {
        Self {
            raw: NonNull::new(ptr).unwrap(),
        }
    }
}
