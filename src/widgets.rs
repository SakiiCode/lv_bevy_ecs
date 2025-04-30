use bevy_ecs::{component::Component, entity::Entity, world::World};
use lvgl::{LvError, LvResult};
use lvgl_sys::{lv_disp_get_scr_act, lv_obj_del, lv_obj_t};

pub struct Obj {
    pub raw: *mut lvgl_sys::lv_obj_t,
}

unsafe impl Send for Obj {}
unsafe impl Sync for Obj {}

#[derive(Component)]
pub struct Widget {
    pub obj: Obj,
}

impl Widget {
    pub fn new<F>(parent: Option<&Widget>, creator: F) -> LvResult<Widget>
    where
        F: Fn(*mut lv_obj_t) -> *mut lv_obj_t,
    {
        unsafe {
            let parent_ptr = match parent {
                Some(parent_widget) => parent_widget.obj.raw,
                None => lv_disp_get_scr_act(std::ptr::null_mut()),
            };
            let ptr = creator(parent_ptr);
            if let Some(raw) = core::ptr::NonNull::new(ptr) {
                Ok(Widget {
                    obj: (Obj { raw: raw.as_ptr() }),
                })
            } else {
                Err(LvError::InvalidReference)
            }
        }
    }

    pub fn get(entity: Entity, world: &World) -> &Self {
        world.entity(entity).get::<Widget>().unwrap()
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

macro_rules! add_create {
    ($t:ident, $func:path) => {
        impl $t {
            pub fn new_widget(parent: Option<Entity>, world: &mut World) -> LvResult<Widget> {
                let parent_widget = parent.map(|e| world.entity(e).get::<Widget>().unwrap());
                let new_widget = Widget::new(parent_widget, Self::create)?;
                Ok(new_widget)
            }

            pub fn spawn_entity(parent: Option<Entity>, world: &mut World) -> LvResult<Entity> {
                let new_widget = Self::new_widget(parent, world)?;
                let new_entity = match parent {
                    Some(parent_entity) => world
                        .entity_mut(parent_entity)
                        .with_child((new_widget, $t))
                        .id(),
                    None => world.spawn((new_widget, $t)).id(),
                };
                return Ok(new_entity);
            }

            fn create(parent: *mut lv_obj_t) -> *mut lv_obj_t {
                unsafe { $func(parent) }
            }
        }
    };
}

#[derive(Component)]
pub struct ButtonComponent;

add_create!(ButtonComponent, lvgl_sys::lv_btn_create);

#[derive(Component)]
pub struct LabelComponent;

add_create!(LabelComponent, lvgl_sys::lv_label_create);

/*
impl LabelComponent {
    pub fn new(parent: Option<Entity>, world: &mut World) -> LvResult<impl Bundle> {
        let parent_widget = parent.map(|e| world.entity(e).get::<Widget>().unwrap());
        return Ok((Widget::new(parent_widget, Self::create)?, LabelComponent));
    }

    fn create(parent: *mut lv_obj_t) -> *mut lv_obj_t {
        unsafe { lvgl_sys::lv_label_create(parent) }
    }
}

impl ButtonComponent {
    pub fn new(parent: Option<Entity>, world: &mut World) -> LvResult<impl Bundle> {
        let parent_widget = parent.map(|e| world.entity(e).get::<Widget>().unwrap());
        return Ok((Widget::new(parent_widget, Self::create)?, ButtonComponent));
    }

    fn create(parent: *mut lv_obj_t) -> *mut lv_obj_t {
        unsafe { lvgl_sys::lv_btn_create(parent) }
    }
}
*/
