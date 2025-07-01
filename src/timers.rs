use std::{ffi::c_void, ptr::NonNull};

use lvgl_sys::lv_timer_t;

use crate::support::LvError;

#[allow(dead_code)]
pub struct Timer {
    raw: NonNull<lvgl_sys::lv_timer_t>,
}

impl Timer {
    pub fn new<F>(callback: F, period: u32) -> Result<Self, LvError>
    where
        F: FnMut(*mut lv_timer_t),
    {
        unsafe {
            let timer = lvgl_sys::lv_timer_create(
                Some(timer_trampoline::<F>),
                period,
                Box::into_raw(Box::new(callback)) as *mut _,
            );
            if let Some(ptr) = NonNull::new(timer) {
                Ok(Self { raw: ptr })
            } else {
                Err(LvError::InvalidReference)
            }
        }
    }
}

unsafe extern "C" fn timer_trampoline<F>(timer: *mut lv_timer_t)
where
    F: FnMut(*mut lv_timer_t),
{
    unsafe {
        let callback = &mut *((*timer).user_data as *mut F);
        callback(timer);
    }
}

pub fn lv_async_call<F>(callback: F)
where
    F: FnMut(),
{
    unsafe {
        lvgl_sys::lv_async_call(
            Some(async_call_trampoline::<F>),
            Box::into_raw(Box::new(callback)) as *mut _,
        );
    }
}

unsafe extern "C" fn async_call_trampoline<F>(obj: *mut c_void)
where
    F: FnMut(),
{
    unsafe {
        let callback = &mut *((obj) as *mut F);
        callback();
    }
}
