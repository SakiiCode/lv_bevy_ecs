use std::ptr::NonNull;

use cty::c_void;
use embedded_graphics::prelude::Point;

/// Pointer-specific input data. Contains the point clicked and the key.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PointerInputData {
    Touch(Point),
    Key(u32),
}

impl PointerInputData {
    pub fn pressed(self) -> InputState {
        InputState::Pressed(Data::Pointer(self))
    }

    pub fn released(self) -> InputState {
        InputState::Released(Data::Pointer(self))
    }
}

/// Generic data which can be associated with an input device driver. Varies
/// based on the concrete type of the input device driver
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Data {
    /// Pointer-specific data.
    Pointer(PointerInputData),
    Encoder,
}

/// Boolean states for an input.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InputState {
    /// Input device key is currently pressed down.
    Pressed(Data),
    /// Input device key is currently released.
    Released(Data),
}

impl InputState {
    /// Represents an input device with one entry in the buffer.
    pub fn once(self) -> BufferStatus {
        BufferStatus::Once(self)
    }
    /// Represents an input device with multiple entries in the buffer.
    pub fn and_continued(self) -> BufferStatus {
        BufferStatus::Buffered(self)
    }
}

/// Boolean buffering states for an input device driver.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum BufferStatus {
    /// One instance of `InputState` remains to be read.
    Once(InputState),
    /// Multiple instances of `InputState` remain to be read.
    Buffered(InputState),
}

pub struct InputDevice {
    raw: NonNull<lvgl_sys::lv_indev_t>,
}

impl InputDevice {
    pub fn create<F>(indev_type: lvgl_sys::lv_indev_type_t, read_cb: F) -> Self
    where
        F: Fn() -> BufferStatus,
    {
        unsafe {
            let raw = NonNull::new(lvgl_sys::lv_indev_create()).unwrap();
            lvgl_sys::lv_indev_set_type(raw.as_ptr(), indev_type);
            lvgl_sys::lv_indev_set_read_cb(raw.as_ptr(), Some(read_input::<F>));
            lvgl_sys::lv_indev_set_user_data(
                raw.as_ptr(),
                Box::into_raw(Box::new(read_cb)) as *mut c_void,
            );

            Self { raw }
        }
    }
}

unsafe extern "C" fn read_input<F>(
    indev: *mut lvgl_sys::lv_indev_t,
    data: *mut lvgl_sys::lv_indev_data_t,
) where
    F: Fn() -> BufferStatus,
{
    unsafe {
        let callback = &mut *((*indev).user_data as *mut F);
        let status = callback();
        let b = match status {
            BufferStatus::Once(state) => {
                (*data).continue_reading = false;
                state
            }
            BufferStatus::Buffered(state) => {
                (*data).continue_reading = true;
                state
            }
        };
        (*data).state = match b {
            InputState::Pressed(d) => {
                match d {
                    Data::Pointer(PointerInputData::Touch(point)) => {
                        (*data).point.x = point.x as lvgl_sys::lv_coord_t;
                        (*data).point.y = point.y as lvgl_sys::lv_coord_t;
                    }
                    Data::Pointer(PointerInputData::Key(_)) => {}
                    _ => panic!("Non-pointer data returned from pointer device!"),
                }
                lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_PRESSED
            }
            InputState::Released(d) => {
                match d {
                    Data::Pointer(PointerInputData::Touch(point)) => {
                        (*data).point.x = point.x as lvgl_sys::lv_coord_t;
                        (*data).point.y = point.y as lvgl_sys::lv_coord_t;
                    }
                    Data::Pointer(PointerInputData::Key(_)) => {}
                    _ => panic!("Non-pointer data returned from pointer device!"),
                }
                lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_RELEASED
            }
        };
    }
}
