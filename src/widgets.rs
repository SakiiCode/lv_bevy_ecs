use std::ptr::NonNull;

use bevy_ecs::{
    component::Component,
    hierarchy::{ChildOf, Children},
    observer::Trigger,
    system::Query,
    world::OnInsert,
};
use lvgl_sys::lv_obj_delete;
use crate::LvError;

#[derive(Component)]
pub struct Widget {
    pub raw: NonNull<lvgl_sys::lv_obj_t>,
}

impl Widget {
    pub fn raw(&self) -> NonNull<lvgl_sys::lv_obj_t> {
        self.raw
    }

    pub fn from_raw(ptr: *mut lvgl_sys::lv_obj_t) -> Option<Self> {
        NonNull::new(ptr).map(|raw| Self { raw })
    }
}

unsafe impl Send for Widget {}
unsafe impl Sync for Widget {}

impl Drop for Widget {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping Obj");
            lv_obj_delete(self.raw.as_ptr());
        }
    }
}

macro_rules! impl_widget {
    ($t:ident, $func:path) => {
        #[derive(Component)]
        pub struct $t;

        impl $t {
            #[allow(dead_code)]
            pub fn create_widget() -> Result<Widget, LvError> {
                unsafe {
                    let default_screen =
                        lvgl_sys::lv_display_get_screen_active(lvgl_sys::lv_display_get_default());
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

impl_widget!(Button, lvgl_sys::lv_button_create);

impl_widget!(Label, lvgl_sys::lv_label_create);

impl_widget!(Keyboard, lvgl_sys::lv_keyboard_create);

impl_widget!(Menu, lvgl_sys::lv_menu_create);

impl_widget!(Dropdown, lvgl_sys::lv_dropdown_create);

impl_widget!(Led, lvgl_sys::lv_dropdown_create);

impl_widget!(Arc, lvgl_sys::lv_arc_create);

impl_widget!(Table, lvgl_sys::lv_table_create);

impl_widget!(Checkbox, lvgl_sys::lv_checkbox_create);

impl_widget!(Bar, lvgl_sys::lv_bar_create);

impl_widget!(Roller, lvgl_sys::lv_roller_create);

impl_widget!(Canvas, lvgl_sys::lv_canvas_create);

impl_widget!(Calendar, lvgl_sys::lv_calendar_create);

impl_widget!(Line, lvgl_sys::lv_line_create);

impl_widget!(Spinbox, lvgl_sys::lv_spinbox_create);

impl_widget!(TileView, lvgl_sys::lv_tileview_create);

impl_widget!(Img, lvgl_sys::lv_image_create);

impl_widget!(Imgbtn, lvgl_sys::lv_imagebutton_create);

impl_widget!(Switch, lvgl_sys::lv_switch_create);

impl_widget!(Chart, lvgl_sys::lv_chart_create);

impl_widget!(Animimg, lvgl_sys::lv_animimg_create);

impl_widget!(Spangroup, lvgl_sys::lv_spangroup_create);

impl_widget!(Btnmatrix, lvgl_sys::lv_buttonmatrix_create);

impl_widget!(Textarea, lvgl_sys::lv_textarea_create);

impl_widget!(Slider, lvgl_sys::lv_slider_create);

impl_widget!(List, lvgl_sys::lv_list_create);
