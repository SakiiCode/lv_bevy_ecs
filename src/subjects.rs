//! # Subjects
//!
//! Subjects don't yet have that `.shorthand()` methods that widgets have.
//! The original LVGL function syntax need to be used.
//! The `lv_bevy_ecs::functions::*` module contains some safe wrappers.
//!
//! ```rust
//! # use lv_bevy_ecs::functions::*;
//! # use lv_bevy_ecs::subjects::{Subject};
//! # use lv_bevy_ecs::widgets::*;
//! # use lv_bevy_ecs::sys::{lv_subject_get_int, lv_observer_get_target, lv_obj_t, lv_dropdown_bind_value,
//! #    lv_chart_type_t_LV_CHART_TYPE_LINE, lv_chart_type_t_LV_CHART_TYPE_BAR};
//! #
//! # lv_bevy_ecs::setup_test_display!();
//! #
//! let mut dropdown = Dropdown::new();
//! let mut chart_type_subject = Subject::new_int(0);
//!
//! dropdown.bind_value(chart_type_subject.raw_mut());
//!
//! let mut chart = Chart::new();
//! lv_subject_add_observer_obj(&mut chart_type_subject, &mut chart, |observer, subject| unsafe {
//!         let v = lv_subject_get_int(subject);
//!         let mut chart_wdg = Wdg::from_ptr(lv_observer_get_target(observer) as *mut lv_obj_t);
//!         let chart: &mut Chart<Wdg> = chart_wdg.downcast_mut().unwrap();
//!         let chart_type = if v == 0 {
//!             lv_chart_type_t_LV_CHART_TYPE_LINE
//!         } else {
//!             lv_chart_type_t_LV_CHART_TYPE_BAR
//!         };
//!         chart.set_type(chart_type);
//! });
//! lv_subject_set_int(&mut chart_type_subject, 1);
//!
//! assert_eq!(chart.get_type(), lv_chart_type_t_LV_CHART_TYPE_BAR);
//! ```

use ::core::{
    ffi::{CStr, c_char, c_void},
    mem::MaybeUninit,
};
use alloc::{boxed::Box, vec};

use bevy_ecs::component::Component;
use lightvgl_sys::lv_subject_t;

use crate::{info, warn, widgets::Wdg};

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Subject {
    raw: lv_subject_t,
}

impl Drop for Subject {
    fn drop(&mut self) {
        unsafe {
            info!("Dropping Subject");
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
                vec![zero; len].into_boxed_slice().as_mut_ptr(),
                core::ptr::null_mut(),
                len,
                value.as_ptr(),
            );
            Self {
                raw: subject.assume_init(),
            }
        }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
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

// the order of parameters is not the same, but callback should come last for readability
pub(crate) fn lv_subject_add_observer_obj<'a, F>(
    subject: &'a mut Subject,
    object: &mut Wdg,
    callback: F,
) where
    F: FnMut(*mut lightvgl_sys::lv_observer_t, *mut lightvgl_sys::lv_subject_t) + 'a,
{
    unsafe {
        lightvgl_sys::lv_subject_add_observer_obj(
            &mut subject.raw,
            Some(subject_callback::<F>),
            object.raw_mut(),
            Box::into_raw(Box::new(callback)) as *mut c_void,
        );
    }
    info!("Added Observer");
}

unsafe extern "C" fn subject_callback<F>(
    observer: *mut lightvgl_sys::lv_observer_t,
    subject: *mut lightvgl_sys::lv_subject_t,
) where
    F: FnMut(*mut lightvgl_sys::lv_observer_t, *mut lightvgl_sys::lv_subject_t),
{
    unsafe {
        if !(*observer).user_data.is_null() {
            let callback = &mut *((*observer).user_data as *mut F);
            callback(observer, subject);
        } else {
            warn!("Subject callback user data was null, this should never happen!");
        }
    }
}
