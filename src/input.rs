//! Input devices
use ::alloc::boxed::Box;
use ::core::{marker::PhantomData, ptr::NonNull};
use lightvgl_sys::{
    lv_indev_get_user_data,
    lv_indev_state_t::{self, LV_INDEV_STATE_RELEASED},
    lv_indev_t, lv_indev_type_t,
};

use embedded_graphics::prelude::Point;

/// Boolean buffering states for an input device driver.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum BufferStatus {
    /// One instance of `InputState` remains to be read.
    #[default]
    Once,
    /// Multiple instances of `InputState` remain to be read.
    Buffered,
}

#[derive(Clone, Copy)]
pub struct InputEvent<T: InputType> {
    pub status: BufferStatus,
    pub state: lv_indev_state_t::Type,
    pub data: T::DataType,
}

impl<T: InputType> InputEvent<T> {
    #[inline]
    pub const fn new(data: T::DataType) -> Self {
        InputEvent {
            status: BufferStatus::Once,
            state: LV_INDEV_STATE_RELEASED,
            data,
        }
    }
}

pub trait InputType {
    type DataType;
    fn as_lv_indev_type() -> lv_indev_type_t::Type;
    fn set_lv_indev_data(event_data: &Self::DataType, data: &mut lightvgl_sys::lv_indev_data_t);
}

#[derive(Clone, Copy, Default)]
pub struct Pointer;

impl InputType for Pointer {
    type DataType = Point;

    #[inline]
    fn as_lv_indev_type() -> lv_indev_type_t::Type {
        lightvgl_sys::lv_indev_type_t::LV_INDEV_TYPE_POINTER
    }

    #[inline]
    fn set_lv_indev_data(event_data: &Self::DataType, data: &mut lightvgl_sys::lv_indev_data_t) {
        data.point.x = event_data.x;
        data.point.y = event_data.y;
    }
}

pub struct Keypad;

impl InputType for Keypad {
    type DataType = u32;

    #[inline]
    fn as_lv_indev_type() -> lv_indev_type_t::Type {
        lightvgl_sys::lv_indev_type_t::LV_INDEV_TYPE_KEYPAD
    }

    #[inline]
    fn set_lv_indev_data(event_data: &Self::DataType, data: &mut lightvgl_sys::lv_indev_data_t) {
        data.key = *event_data;
    }
}

pub struct Encoder;

impl InputType for Encoder {
    type DataType = i16;

    #[inline]
    fn as_lv_indev_type() -> lv_indev_type_t::Type {
        lightvgl_sys::lv_indev_type_t::LV_INDEV_TYPE_ENCODER
    }

    #[inline]
    fn set_lv_indev_data(event_data: &Self::DataType, data: &mut lightvgl_sys::lv_indev_data_t) {
        data.enc_diff = *event_data;
    }
}
pub struct Button;

impl InputType for Button {
    type DataType = u32;

    #[inline]
    fn as_lv_indev_type() -> lv_indev_type_t::Type {
        lightvgl_sys::lv_indev_type_t::LV_INDEV_TYPE_BUTTON
    }

    #[inline]
    fn set_lv_indev_data(event_data: &Self::DataType, data: &mut lightvgl_sys::lv_indev_data_t) {
        data.btn_id = *event_data;
    }
}

pub struct InputDevice<T: InputType> {
    raw: NonNull<lv_indev_t>,
    r#type: PhantomData<T>,
}

impl<T: InputType> InputDevice<T> {
    pub fn new<F>(read_cb: F) -> Self
    where
        F: FnMut() -> InputEvent<T>,
    {
        unsafe {
            let raw = NonNull::new(lightvgl_sys::lv_indev_create()).unwrap();
            lightvgl_sys::lv_indev_set_type(raw.as_ptr(), T::as_lv_indev_type());
            lightvgl_sys::lv_indev_set_read_cb(raw.as_ptr(), Some(read_input::<F, T>));
            lightvgl_sys::lv_indev_set_user_data(
                raw.as_ptr(),
                Box::into_raw(Box::new(read_cb)).cast(),
            );

            Self {
                raw,
                r#type: PhantomData,
            }
        }
    }

    #[inline]
    pub fn raw(&self) -> *const lv_indev_t {
        self.raw.as_ptr().cast_const()
    }

    #[inline]
    pub fn raw_mut(&mut self) -> *mut lv_indev_t {
        self.raw.as_ptr()
    }
}

unsafe extern "C" fn read_input<F, T>(
    indev: *mut lv_indev_t,
    data: *mut lightvgl_sys::lv_indev_data_t,
) where
    T: InputType,
    F: FnMut() -> InputEvent<T>,
{
    unsafe {
        let user_data = lv_indev_get_user_data(indev);
        if !user_data.is_null() {
            let callback = &mut *(user_data.cast::<F>());
            let event = callback();
            match event.status {
                BufferStatus::Once => {
                    (*data).continue_reading = false;
                }
                BufferStatus::Buffered => {
                    (*data).continue_reading = true;
                }
            }
            T::set_lv_indev_data(&event.data, data.as_mut().unwrap());
            (*data).state = event.state;
        } else {
            crate::warn!("Input callback user data was null, this should never happen!");
        }
    }
}
