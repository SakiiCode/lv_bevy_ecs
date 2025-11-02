//! # Animations
//!
//! Animations are components that need to be added to entities
//!
//! ```
//! use std::time::Duration;
//! use lv_bevy_ecs::animation::Animation;
//! use lv_bevy_ecs::functions::*;
//! use lv_bevy_ecs::support::OpacityLevel;
//! use lv_bevy_ecs::sys::{lv_part_t_LV_PART_MAIN};
//! use lv_bevy_ecs::widgets::{Button, LvglWorld};
//!
//! lv_bevy_ecs::setup_test_display!();
//!
//! let mut world = LvglWorld::new();
//! let button = Button::create_widget();
//!
//! let anim = Animation::new(
//!     Duration::from_secs(5),
//!     OpacityLevel::Transparent as i32,
//!     OpacityLevel::Cover as i32,
//!     |obj, val| {
//!         lv_obj_set_style_opa(obj, val as u8, lv_part_t_LV_PART_MAIN);
//!     },
//! );
//! let mut button_entity = world.spawn((Button, button, anim));
//! unsafe {
//!     assert_eq!(lv_bevy_ecs::sys::lv_anim_count_running(), 1);
//! }
//! ```

use std::{ffi::c_void, ptr::NonNull, time::Duration};

use crate::info;
use bevy_ecs::{component::Component, lifecycle::HookContext, world::DeferredWorld};

use crate::widgets::{Wdg, Widget};

#[derive(Component)]
#[component(on_insert=add_animation)]
pub struct Animation {
    raw: Option<lightvgl_sys::lv_anim_t>,
}

impl Animation {
    pub fn new<F>(duration: Duration, start: i32, end: i32, animator: F) -> Self
    where
        F: FnMut(&mut Wdg, i32),
    {
        let mut raw = unsafe {
            let mut anim = std::mem::MaybeUninit::<lightvgl_sys::lv_anim_t>::uninit();
            lightvgl_sys::lv_anim_init(anim.as_mut_ptr());
            anim.assume_init()
        };
        raw.duration = duration.as_millis().try_into().unwrap_or(0);
        raw.start_value = start;
        raw.current_value = start;
        raw.end_value = end;
        raw.user_data = Box::<F>::into_raw(Box::new(animator)) as *mut _;
        raw.exec_cb = Some(animator_trampoline::<F>);

        Self { raw: Some(raw) }
    }

    pub fn start(&mut self) {
        unsafe {
            self.raw = Some(*lightvgl_sys::lv_anim_start(&self.raw.take().unwrap()));
        }
    }

    pub fn raw(&self) -> &lightvgl_sys::lv_anim_t {
        self.raw.as_ref().unwrap()
    }

    pub fn raw_mut(&mut self) -> &mut lightvgl_sys::lv_anim_t {
        self.raw.as_mut().unwrap()
    }
}

unsafe impl Send for Animation {}
unsafe impl Sync for Animation {}

impl Drop for Animation {
    fn drop(&mut self) {
        info!("Dropping Animation");
    }
}

fn add_animation(mut world: DeferredWorld, ctx: HookContext) {
    let obj = world
        .get_mut::<Widget>(ctx.entity)
        .expect("Animation components must be added entities having a Widget component")
        .as_mut()
        .raw();
    let mut anim = world.get_mut::<Animation>(ctx.entity).unwrap();
    anim.raw.as_mut().unwrap().var = obj as *mut _;
    anim.start();
    info!("Added Animation");
}

unsafe extern "C" fn animator_trampoline<F>(obj: *mut c_void, val: i32)
where
    F: FnMut(&mut Wdg, i32),
{
    unsafe {
        let anim =
            NonNull::new(lightvgl_sys::lv_anim_get(obj, None) as *mut lightvgl_sys::lv_anim_t)
                .unwrap();
        // yes, we have to do it this way. Casting `obj` directly to `&mut Obj` segfaults
        let obj = obj as *mut lightvgl_sys::lv_obj_t;
        if !anim.as_ref().user_data.is_null() {
            let callback = &mut *(anim.as_ref().user_data as *mut F);
            let mut obj_nondrop = Widget::from_ptr(obj).unwrap();
            callback(&mut obj_nondrop, val);
            std::mem::forget(obj_nondrop)
        }
    }
}
