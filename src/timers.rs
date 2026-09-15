//! # Timers
//!
//! Timers are components, can be used as a standalone entity or attached to another entity
//! ```
//! # use lv_bevy_ecs::widgets::LvglWorld;
//! # use lv_bevy_ecs::timers::Timer;
//! # use lv_bevy_ecs::sys::lv_timer_get_next;
//! #
//! # lv_bevy_ecs::functions::lv_init();
//! # let mut world = LvglWorld::default();
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
//! # use lv_bevy_ecs::functions::{lv_async_call, lv_init};
//! # lv_init();
//! lv_async_call(||{
//!     // ...
//! })
//! ```

use ::alloc::boxed::Box;
use ::core::{ptr::NonNull, time::Duration};

#[cfg(feature = "ecs")]
use bevy_ecs::{
    component::Component,
    schedule::{IntoScheduleConfigs, Schedule},
    system::ScheduleSystem,
    world::World,
};
use lightvgl_sys::{lv_timer_get_user_data, lv_timer_t};

#[cfg_attr(feature = "ecs", derive(Component))]
#[cfg_attr(feature = "ecs", component(storage = "SparseSet"))]
pub struct Timer {
    raw: NonNull<lv_timer_t>,
    schedule: Schedule,
}

impl Drop for Timer {
    fn drop(&mut self) {
        unsafe {
            crate::info!("Dropping Timer");
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
                u32::try_from(period.as_millis()).unwrap(),
                Box::into_raw(Box::new((&mut schedule, world))).cast(),
            );
            let ptr = NonNull::new(timer);
            Some(Self {
                raw: ptr?,
                schedule,
            })
        }
    }

    #[expect(clippy::impl_trait_in_params)]
    #[inline]
    pub fn add_systems<M>(&mut self, system: impl IntoScheduleConfigs<ScheduleSystem, M>) {
        self.schedule.add_systems(system);
    }
}

unsafe extern "C" fn timer_trampoline(timer: *mut lv_timer_t) {
    unsafe {
        let user_data = lv_timer_get_user_data(timer);
        if !user_data.is_null() {
            let (schedule, world) = &mut *(user_data.cast::<(&mut Schedule, &mut World)>());
            schedule.run(world);
        } else {
            crate::warn!("Timer callback user data was null, this should never happen!");
        }
    }
}
