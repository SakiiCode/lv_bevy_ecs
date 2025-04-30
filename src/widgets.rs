use bevy_ecs::component::Component;
use lvgl::{LvError, LvResult};
use lvgl_sys::{_lv_obj_t, lv_obj_del};

pub struct Obj {
  pub raw: *mut lvgl_sys::lv_obj_t,
}

unsafe impl Send for Obj {}
unsafe impl Sync for Obj {}



#[derive(Component)]
pub struct Widget {
    pub obj: Obj,
}

#[derive(Component)]
pub struct Button;

#[derive(Component)]
pub struct Label;

impl Button {
  pub fn new(parent: &mut _lv_obj_t) -> LvResult<Widget> {
      unsafe {
          let ptr = lvgl_sys::lv_btn_create(parent);
          if let Some(raw) = core::ptr::NonNull::new(ptr) {
              Ok(Widget {
                  obj: (Obj { raw: raw.as_ptr() }),
              })
          } else {
              Err(LvError::InvalidReference)
          }
      }
  }
}

impl Label {
  pub fn new(parent: &mut _lv_obj_t) -> LvResult<Widget> {
      unsafe {
          let ptr = lvgl_sys::lv_label_create(parent);
          if let Some(raw) = core::ptr::NonNull::new(ptr) {
              Ok(Widget {
                  obj: (Obj { raw: raw.as_ptr() }),
              })
          } else {
              Err(LvError::InvalidReference)
          }
      }
  }
}

impl Drop for Widget {
  fn drop(&mut self) {
      unsafe {
          println!("Dropping Widget");
          lv_obj_del(self.obj.raw);
      }
  }
}
