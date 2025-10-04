//! Input
use std::{marker::PhantomData, ptr::NonNull};

use cty::c_void;
use embedded_graphics::prelude::Point;

pub trait InputData<T: LvglInputType> {
    fn set_lv_indev_data(&self, data: &mut lightvgl_sys::lv_indev_data_t);
}

impl InputData<PointerInputDevice> for Point {
    fn set_lv_indev_data(&self, data: &mut lightvgl_sys::lv_indev_data_t) {
        data.point.x = self.x;
        data.point.y = self.y;
    }
}

impl InputData<KeypadInputDevice> for u32 {
    fn set_lv_indev_data(&self, data: &mut lightvgl_sys::lv_indev_data_t) {
        data.key = *self
    }
}

/// Boolean states for an input.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InputState {
    /// Input device key is currently pressed down.
    Pressed,
    /// Input device key is currently released.
    Released,
}

impl InputState {
    fn as_lv_indev_state(&self) -> lightvgl_sys::lv_indev_state_t {
        match self {
            InputState::Pressed => lightvgl_sys::lv_indev_state_t_LV_INDEV_STATE_PRESSED,
            InputState::Released => lightvgl_sys::lv_indev_state_t_LV_INDEV_STATE_RELEASED,
        }
    }
}

/// Boolean buffering states for an input device driver.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum BufferStatus {
    /// One instance of `InputState` remains to be read.
    Once,
    /// Multiple instances of `InputState` remain to be read.
    Buffered,
}

#[derive(Clone, Copy)]
pub struct LvglInputEvent<T: LvglInputType, D: InputData<T>> {
    pub status: BufferStatus,
    pub state: InputState,
    pub data: D,
    pub device_type: PhantomData<T>,
}

pub trait LvglInputType {
    fn as_lv_indev_type() -> lightvgl_sys::lv_indev_type_t;
}

#[derive(Clone, Copy)]
pub struct PointerInputDevice;

impl LvglInputType for PointerInputDevice {
    fn as_lv_indev_type() -> lightvgl_sys::lv_indev_type_t {
        lightvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_POINTER
    }
}

pub struct KeypadInputDevice;

impl LvglInputType for KeypadInputDevice {
    fn as_lv_indev_type() -> lightvgl_sys::lv_indev_type_t {
        lightvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_KEYPAD
    }
}
pub struct EncoderInputDevice;

impl LvglInputType for EncoderInputDevice {
    fn as_lv_indev_type() -> lightvgl_sys::lv_indev_type_t {
        lightvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_ENCODER
    }
}
pub struct ButtonInputDevice;

impl LvglInputType for ButtonInputDevice {
    fn as_lv_indev_type() -> lightvgl_sys::lv_indev_type_t {
        lightvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_BUTTON
    }
}

#[allow(dead_code)]
pub struct InputDevice<T: LvglInputType> {
    raw: NonNull<lightvgl_sys::lv_indev_t>,
    r#type: PhantomData<T>,
}

impl<T: LvglInputType> InputDevice<T> {
    pub fn create<F, D>(read_cb: F) -> Self
    where
        D: InputData<T>,
        F: FnMut() -> LvglInputEvent<T, D>,
    {
        unsafe {
            let raw = NonNull::new(lightvgl_sys::lv_indev_create()).unwrap();
            lightvgl_sys::lv_indev_set_type(raw.as_ptr(), T::as_lv_indev_type());
            lightvgl_sys::lv_indev_set_read_cb(raw.as_ptr(), Some(read_input::<F, T, D>));
            lightvgl_sys::lv_indev_set_user_data(
                raw.as_ptr(),
                Box::into_raw(Box::new(read_cb)) as *mut c_void,
            );

            Self {
                raw,
                r#type: PhantomData,
            }
        }
    }
}

unsafe extern "C" fn read_input<F, T, D>(
    indev: *mut lightvgl_sys::lv_indev_t,
    data: *mut lightvgl_sys::lv_indev_data_t,
) where
    T: LvglInputType,
    D: InputData<T>,
    F: FnMut() -> LvglInputEvent<T, D>,
{
    unsafe {
        let callback = &mut *((*indev).user_data as *mut F);
        let event = callback();
        match event.status {
            BufferStatus::Once => {
                (*data).continue_reading = false;
            }
            BufferStatus::Buffered => {
                (*data).continue_reading = true;
            }
        };
        //D::set_lv_indev_data(&self, data);
        event.data.set_lv_indev_data(data.as_mut().unwrap());
        (*data).state = event.state.as_lv_indev_state();
    }
}
