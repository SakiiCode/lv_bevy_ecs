//! # Timers
//!
//! Timers are components, can be used as a standalone entity or attached to another entity
//! ```
//! # use lv_bevy_ecs::widgets::LvglWorld;
//! # use lv_bevy_ecs::timers::Timer;
//! # use lv_bevy_ecs::sys::lv_timer_get_next;
//! #
//! #
//! # let mut world = LvglWorld::new();
//! #
//! let mut timer = Timer::new(
//!     &mut world,
//!     std::time::Duration::from_millis(5000),
//! ).unwrap();
//!
//! timer.add_systems(||{
//!     // ...
//! });
//!
//! world.spawn(timer);
//!
//! unsafe {
//!     assert_ne!(lv_timer_get_next(core::ptr::null_mut()), core::ptr::null_mut());
//! }
//! ```
//! To delete a timer, despawn the entity or remove the component and it will be automatically dropped.
//!
//! ## Async calls
//!
//! Closure will be executed on the next `lv_timer_handler()`. It needs `'static` lifetime.
//! ```
//! # use lv_bevy_ecs::functions::lv_async_call;
//! #
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

use crate::info;

#[allow(dead_code)]
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Timer {
    raw: NonNull<lv_timer_t>,
    schedule: Schedule,
}

impl Drop for Timer {
    fn drop(&mut self) {
        unsafe {
            info!("Dropping Timer");
            lightvgl_sys::lv_timer_delete(self.raw.as_ptr());
        }
    }
}

unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}

impl Timer {
    pub fn new(world: &mut World, period: Duration) -> Option<Self> {
        let mut schedule = Schedule::default();
        unsafe {
            let timer = lightvgl_sys::lv_timer_create(
                Some(timer_trampoline),
                period.as_millis() as u32,
                Box::into_raw(Box::new((&mut schedule, world))) as *mut _,
            );
            let ptr = NonNull::new(timer);
            Some(Self {
                raw: ptr?,
                schedule,
            })
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

pub(crate) fn lv_async_call<F>(callback: F)
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
