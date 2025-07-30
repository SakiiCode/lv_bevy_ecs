//! # Timers
//!
//! Timers are components, can be used as a standalone entity or attached to another entity
//! ```rust
//! let timer = Timer::new(
//!     move |_timer| {
//!         // ...
//!     },
//!     Duration::from_millis(5000),
//! )?;
//! world.spawn(timer);
//! ```
//! To delete a timer, despawn the entity or remove the component and it will be automatically dropped.
//!
//! ## Async calls
//!
//! Closure will be executed on the next `lv_timer_handler()`. It needs `'static` lifetime.
//! ```rust
//! lv_async_call(||{
//!     // ...
//! })
//! ```

use std::{ffi::c_void, ptr::NonNull, time::Duration};

use bevy_ecs::component::Component;
use lightvgl_sys::lv_timer_t;

use crate::support::LvError;

#[allow(dead_code)]
#[derive(Component)]
pub struct Timer {
    raw: NonNull<lv_timer_t>,
}

impl Drop for Timer {
    fn drop(&mut self) {
        unsafe {
            lightvgl_sys::lv_timer_delete(self.raw.as_ptr());
        }
    }
}

unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}

impl Timer {
    pub fn new<F>(callback: F, period: Duration) -> Result<Self, LvError>
    where
        F: FnMut(*mut lv_timer_t) + 'static,
    {
        unsafe {
            let timer = lightvgl_sys::lv_timer_create(
                Some(timer_trampoline::<F>),
                period.as_millis() as u32,
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
    F: FnMut() + 'static,
{
    unsafe {
        lightvgl_sys::lv_async_call(
            Some(async_call_trampoline::<F>),
            Box::into_raw(Box::new(callback)) as *mut _,
        );
    }
}

unsafe extern "C" fn async_call_trampoline<F>(obj: *mut c_void)
where
    F: FnMut() + 'static,
{
    unsafe {
        let callback = &mut *((obj) as *mut F);
        callback();
    }
}
