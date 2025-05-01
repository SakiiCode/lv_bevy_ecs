use std::{ffi::c_void, ptr::NonNull, time::Duration};

use bevy_ecs::{
    component::{Component, HookContext},
    world::DeferredWorld,
};

use crate::widgets::Widget;

#[derive(Component)]
#[component(on_insert=add_animation)]
pub struct Animation {
    pub raw: Box<lvgl_sys::lv_anim_t>,
}

impl Drop for Animation {
    fn drop(&mut self) {
        println!("Dropping Animation");
    }
}

impl Animation {
    pub fn new<F>(duration: Duration, start: i32, end: i32, animator: F) -> Self
    where
        F: FnMut(&mut Widget, i32),
    {
        let mut raw = unsafe {
            let mut anim = std::mem::MaybeUninit::<lvgl_sys::lv_anim_t>::uninit();
            lvgl_sys::lv_anim_init(anim.as_mut_ptr());
            Box::new(anim.assume_init())
        };
        raw.time = duration.as_millis().try_into().unwrap_or(0);
        raw.start_value = start;
        raw.current_value = start;
        raw.end_value = end;
        raw.user_data = Box::<F>::into_raw(Box::new(animator)) as *mut _;
        raw.exec_cb = Some(animator_trampoline::<F>);

        Self { raw }
    }

    /// Starts the animation.
    pub fn start(&mut self) {
        unsafe {
            self.raw = Box::from_raw(lvgl_sys::lv_anim_start(self.raw.as_mut()));
        }
    }
}

unsafe impl Send for Animation {}
unsafe impl Sync for Animation {}

pub fn add_animation(mut world: DeferredWorld, ctx: HookContext) {
    let widget = world
        .get_mut::<Widget>(ctx.entity)
        .expect("Animation components must be added to Widget entities").as_mut() as *mut Widget;
    let mut anim = world.get_mut::<Animation>(ctx.entity).unwrap();
    anim.raw.var = widget as *mut _;
    anim.start();
    dbg!("Added style");
}

unsafe extern "C" fn animator_trampoline<F>(obj: *mut c_void, val: i32)
where
    F: FnMut(&mut Widget, i32),
{
    unsafe {
        let anim =
            NonNull::new(lvgl_sys::lv_anim_get(obj, None) as *mut lvgl_sys::lv_anim_t).unwrap();
        // yes, we have to do it this way. Casting `obj` directly to `&mut Obj` segfaults
        let obj = (*(obj as *mut Widget)).raw();
        if !anim.as_ref().user_data.is_null() {
            let callback = &mut *(anim.as_ref().user_data as *mut F);
            let mut obj_nondrop = Widget::from_raw(obj);
            callback(&mut obj_nondrop, val);
            std::mem::forget(obj_nondrop)
        }
    }
}
