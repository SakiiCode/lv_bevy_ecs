use std::ptr::NonNull;

use bevy_ecs::{
    component::Component,
    hierarchy::{ChildOf, Children},
    observer::Trigger,
    system::Query,
    world::OnInsert,
};
use lvgl::LvError;
use lvgl_sys::lv_obj_del;

#[derive(Component)]
pub struct Widget {
    pub raw: NonNull<lvgl_sys::lv_obj_t>,
}

impl Widget {
    pub fn raw(&self) -> NonNull<lvgl_sys::lv_obj_t> {
        self.raw
    }

    pub fn from_raw(ptr: NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self { raw: ptr }
    }
}

unsafe impl Send for Widget {}
unsafe impl Sync for Widget {}

impl Drop for Widget {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping Obj");
            lv_obj_del(self.raw.as_ptr());
        }
    }
}

macro_rules! impl_widget {
    ($t:ident,$bundle:ident, $func:path) => {
        #[derive(Component)]
        pub struct $t;

        impl $t {
            #[allow(dead_code)]
            pub fn create_widget() -> Result<Widget, LvError> {
                unsafe {
                    let default_screen =
                        lvgl_sys::lv_disp_get_scr_act(lvgl_sys::lv_disp_get_default());
                    let ptr = $func(default_screen);
                    if let Some(raw) = core::ptr::NonNull::new(ptr) {
                        Ok(Widget { raw })
                    } else {
                        Err(LvError::InvalidReference)
                    }
                }
            }
        }
    };
}

impl_widget!(Button, ButtonBundle, lvgl_sys::lv_btn_create);

impl_widget!(Label, LabelBundle, lvgl_sys::lv_label_create);

impl_widget!(Keyboard, KeyboardBundle, lvgl_sys::lv_keyboard_create);

impl_widget!(Menu, MenuBundle, lvgl_sys::lv_menu_create);

impl_widget!(Dropdown, DropdownBundle, lvgl_sys::lv_dropdown_create);

pub fn on_insert_children(
    trigger: Trigger<OnInsert, Children>,
    widgets: Query<&Widget>,
    children: Query<(&Widget, &ChildOf)>,
) {
    let mut parent_widget = None;
    for (widget, parent) in children.iter() {
        if parent.parent() == trigger.target() {
            parent_widget = Some(widget);
        }
    }
    let child_ptr = parent_widget.expect("Parent not found").raw.as_ptr();
    let parent_ptr = widgets.get(trigger.target()).unwrap().raw.as_ptr();
    unsafe {
        lvgl_sys::lv_obj_set_parent(child_ptr, parent_ptr);
    }
    dbg!("On Insert Children");
}
