//! # Widgets
//!
//! ```rust
//! let mut label: Widget = Label::create_widget()?;
//! lv_label_set_text(&mut label, cstr!("Example label"));
//! world.spawn((Label, label));
//! ```
//!
//! To create widgets, you have to use "zero-sized marker structs" (Arc, Label, Button, ...) that create `Widget` objects
//! using the `create_widget` function.
//!
//! Most of the LVGL functions have been kept with original names and they will use this `Widget` as their first parameter.
//!
//! In order to ECS to know the type of the Widget, pass the marker struct next to it when spawning the entity. (This is not mandatory but useful for queries)
//!
//! ## Modifying Widgets
//!
//! To access widgets after moving them to the World with the `spawn()` function, you have to use queries
//! ```rust
//! let mut labels = world.query_filtered::<&mut Widget, With<Label>>();
//! for label in labels.iter_mut(){
//!   //...
//! }
//! ```
//!
//! In case of a unique entity:
//! ```rust
//! let mut label = world.query_filtered::<&mut Widget, With<Label>>().single_mut().unwrap();
//! ```
//!
//! You are free to define any kind of custom component:
//!
//! ```rust
//! #[derive(Component)]
//! struct DynamicLabel;
//! // ...
//! world.spawn((Label, label, DynamicLabel));
//! //...
//! let mut label = world.query_filtered::<&mut Widget, With<DynamicLabel>>().single_mut().unwrap();
//! ```
//! 
//! ## Child widgets
//! To add a widget as a child, set it as child entity
//! ```rust
//! let mut button_entity = world.spawn((Button, button));
//! let mut label_entity = button_entity.with_child((Label, label));
//! ```

use std::ptr::NonNull;

use crate::support::LvError;
use bevy_ecs::{
    component::Component, hierarchy::ChildOf, observer::Trigger, system::Query, world::OnInsert,
};
use lvgl_sys::lv_obj_delete;

#[derive(Component)]
pub struct Widget {
    raw: NonNull<lvgl_sys::lv_obj_t>,
}

impl Widget {
    pub fn raw(&self) -> *mut lvgl_sys::lv_obj_t {
        self.raw.as_ptr()
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

pub fn on_insert_parent(
    trigger: Trigger<OnInsert, ChildOf>,
    widgets: Query<&Widget>,
    children: Query<(&Widget, &ChildOf)>,
) {
    let parent_widget = children.get(trigger.target()).unwrap();
    let parent_ptr = widgets.get(parent_widget.1.0).unwrap().raw();
    let child_ptr = children.get(trigger.target()).unwrap().0.raw();
    unsafe {
        lvgl_sys::lv_obj_set_parent(child_ptr, parent_ptr);
    }
    dbg!("On Insert Parent");
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

impl_widget!(Image, lvgl_sys::lv_image_create);

impl_widget!(Imagebutton, lvgl_sys::lv_imagebutton_create);

impl_widget!(Switch, lvgl_sys::lv_switch_create);

impl_widget!(Chart, lvgl_sys::lv_chart_create);

impl_widget!(Animimg, lvgl_sys::lv_animimg_create);

impl_widget!(Spangroup, lvgl_sys::lv_spangroup_create);

impl_widget!(Btnmatrix, lvgl_sys::lv_buttonmatrix_create);

impl_widget!(Textarea, lvgl_sys::lv_textarea_create);

impl_widget!(Slider, lvgl_sys::lv_slider_create);

impl_widget!(List, lvgl_sys::lv_list_create);

#[cfg(feature = "qrcode")]
impl_widget!(QrCode, lvgl_sys::lv_qrcode_create);

#[cfg(feature = "barcode")]
impl_widget!(Barcode, lvgl_sys::lv_barcode_create);
