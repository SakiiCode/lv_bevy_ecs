//! # Timers
//!
//! Timers are components, can be used as a standalone entity or attached to another entity
//! ```ignore
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
//! ```ignore
//! lv_async_call(||{
//!     // ...
//! })
//! ```

use std::{ffi::c_void, ptr::NonNull, time::Duration};

use bevy_ecs::{
    component::Component,
    schedule::{IntoScheduleConfigs, Schedule},
    system::ScheduleSystem,
    world::World,
};
use lightvgl_sys::lv_timer_t;

use crate::support::LvError;

#[allow(dead_code)]
#[derive(Component)]
pub struct Timer {
    raw: NonNull<lv_timer_t>,
    schedule: Schedule,
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
    pub fn new(world: &mut World, period: Duration) -> Result<Self, LvError> {
        let mut schedule = Schedule::default();
        unsafe {
            let timer = lightvgl_sys::lv_timer_create(
                Some(timer_trampoline),
                period.as_millis() as u32,
                Box::into_raw(Box::new((&mut schedule, world))) as *mut _,
            );
            if let Some(ptr) = NonNull::new(timer) {
                Ok(Self { raw: ptr, schedule })
            } else {
                Err(LvError::InvalidReference)
            }
        }
    }

    pub fn add_systems<M>(&mut self, system: impl IntoScheduleConfigs<ScheduleSystem, M>) {
        self.schedule.add_systems(system);
    }
}

unsafe extern "C" fn timer_trampoline(timer: *mut lv_timer_t) {
    unsafe {
        let (schedule, world) = &mut *((*timer).user_data as *mut (&mut Schedule, &mut World));
        schedule.run(world);
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
