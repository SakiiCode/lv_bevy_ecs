use std::{
    ffi::{c_char, c_void, CStr},
    mem::MaybeUninit,
};

use lvgl_sys::lv_subject_t;

pub struct Subject {
    raw: lv_subject_t,
}

impl Subject {
    pub fn new_int(value: i32) -> Self {
        unsafe {
            let mut subject = MaybeUninit::<lvgl_sys::lv_subject_t>::uninit();
            lvgl_sys::lv_subject_init_int(subject.as_mut_ptr(), value);
            Self {
                raw: subject.assume_init(),
            }
        }
    }

    pub fn new_string(value: &CStr) -> Self {
        unsafe {
            let mut subject = MaybeUninit::<lvgl_sys::lv_subject_t>::uninit();
            let len = value.count_bytes();
            let zero: c_char = 0;
            lvgl_sys::lv_subject_init_string(
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
            let mut subject = MaybeUninit::<lvgl_sys::lv_subject_t>::uninit();
            lvgl_sys::lv_subject_init_pointer(subject.as_mut_ptr(), value);
            Self {
                raw: subject.assume_init(),
            }
        }
    }

    pub fn raw(&mut self) -> &mut lv_subject_t {
        &mut self.raw
    }
}
