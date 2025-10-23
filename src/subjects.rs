//! # Subjects
//!
//! ```rust
//! use lv_bevy_ecs::functions::lv_chart_get_type;
//! use lv_bevy_ecs::subjects::{Subject, lv_subject_set_int, lv_subject_add_observer_obj};
//! use lv_bevy_ecs::sys::*;
//! use lv_bevy_ecs::widgets::*;
//!
//! lv_bevy_ecs::setup_test_display!();
//!
//! let mut dropdown = Dropdown::create_widget();
//! let mut chart_type_subject = Subject::new_int(0);
//!
//! unsafe {
//!     lv_bevy_ecs::sys::lv_dropdown_bind_value(dropdown.raw_mut(), chart_type_subject.raw_mut());
//! }
//!
//! let mut chart = Chart::create_widget();
//! lv_subject_add_observer_obj(&mut chart_type_subject, &mut chart, |observer, subject| unsafe {
//!        let v = lv_subject_get_int(subject);
//!        let chart = lv_observer_get_target(observer) as *mut lv_obj_t;
//!        let type_ = if v == 0 {
//!            lv_chart_type_t_LV_CHART_TYPE_LINE
//!        } else {
//!            lv_chart_type_t_LV_CHART_TYPE_BAR
//!        };
//!        lv_chart_set_type(chart, type_);
//! });
//! lv_subject_set_int(&mut chart_type_subject, 1);
//!
//! assert_eq!(lv_chart_get_type(&mut chart), lv_chart_type_t_LV_CHART_TYPE_BAR);
//! ```

use std::{
    ffi::{CStr, c_char, c_void},
    mem::MaybeUninit,
};

use bevy_ecs::component::Component;
use lightvgl_sys::lv_subject_t;

use crate::{trace, widgets::Widget};

#[derive(Component)]
pub struct Subject {
    raw: lv_subject_t,
}

impl Drop for Subject {
    fn drop(&mut self) {
        unsafe {
            lightvgl_sys::lv_subject_deinit(&mut self.raw);
        }
    }
}

unsafe impl Send for Subject {}
unsafe impl Sync for Subject {}

impl Subject {
    pub fn new_int(value: i32) -> Self {
        unsafe {
            let mut subject = MaybeUninit::<lightvgl_sys::lv_subject_t>::uninit();
            lightvgl_sys::lv_subject_init_int(subject.as_mut_ptr(), value);
            Self {
                raw: subject.assume_init(),
            }
        }
    }

    pub fn new_string(value: &CStr) -> Self {
        unsafe {
            let mut subject = MaybeUninit::<lightvgl_sys::lv_subject_t>::uninit();
            let len = value.count_bytes();
            let zero: c_char = 0;
            lightvgl_sys::lv_subject_init_string(
                subject.as_mut_ptr(),
                &mut Box::leak(vec![zero; len].into_boxed_slice())[0],
                std::ptr::null_mut(),
                len,
                value.as_ptr(),
            );
            Self {
                raw: subject.assume_init(),
            }
        }
    }

    pub fn new_ptr(value: *mut c_void) -> Self {
        unsafe {
            let mut subject = MaybeUninit::<lightvgl_sys::lv_subject_t>::uninit();
            lightvgl_sys::lv_subject_init_pointer(subject.as_mut_ptr(), value);
            Self {
                raw: subject.assume_init(),
            }
        }
    }

    pub fn raw_mut(&mut self) -> &mut lv_subject_t {
        &mut self.raw
    }

    pub fn raw(&self) -> &lv_subject_t {
        &self.raw
    }
}

pub(crate) fn lv_subject_add_observer_obj<'a, F>(
    subject: &'a mut Subject,
    object: &mut Widget,
    callback: F,
) where
    F: FnMut(*mut lightvgl_sys::lv_observer_t, *mut lightvgl_sys::lv_subject_t) + 'a,
{
    trace!("lv_subject_add_observer_obj");
    unsafe {
        lightvgl_sys::lv_subject_add_observer_obj(
            &mut subject.raw,
            Some(subject_callback::<F>),
            object.raw_mut(),
            Box::into_raw(Box::new(callback)) as *mut c_void,
        );
    }
}

unsafe extern "C" fn subject_callback<F>(
    observer: *mut lightvgl_sys::lv_observer_t,
    subject: *mut lightvgl_sys::lv_subject_t,
) where
    F: FnMut(*mut lightvgl_sys::lv_observer_t, *mut lightvgl_sys::lv_subject_t),
{
    unsafe {
        let callback = &mut *((*observer).user_data as *mut F);
        callback(observer, subject);
    }
}
