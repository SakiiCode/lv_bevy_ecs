//! # Events
use crate::widgets::Wdg;

/// Events are triggered in LVGL when something happens which might be interesting to
/// the user, e.g. if an object:
///  - is clicked
///  - is dragged
///  - its value has changed, etc.
///
/// All objects (such as Buttons/Labels/Sliders etc.) receive these generic events
/// regardless of their type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Event {
    /// The object has been pressed
    Pressed,

    /// The object is being pressed (sent continuously while pressing)
    Pressing,

    /// The input device is still being pressed but is no longer on the object
    PressLost,

    /// Released before `long_press_time` config time. Not called if dragged.
    ShortClicked,

    /// Called on release if not dragged (regardless to long press)
    Clicked,

    /// Pressing for `long_press_time` config time. Not called if dragged.
    LongPressed,

    /// Called after `long_press_time` config in every `long_press_rep_time` ms. Not
    /// called if dragged.
    LongPressedRepeat,

    /// Called in every case when the object has been released even if it was dragged. Not called
    /// if slid from the object while pressing and released outside of the object. In this
    /// case, `Event<_>::PressLost` is sent.
    Released,

    /// Called when an underlying value is changed e.g. position of a `Slider`.
    ValueChanged,

    ///
    DrawMain,

    ///
    DrawMainBegin,

    ///
    DrawMainEnd,

    ///
    DrawPost,

    ///
    DrawPostBegin,

    ///
    DrawPostEnd,

    /// Called on focus
    Focused,
}

impl TryFrom<lightvgl_sys::lv_event_code_t> for Event {
    type Error = ();

    fn try_from(value: lightvgl_sys::lv_event_code_t) -> Result<Self, Self::Error> {
        const LV_EVENT_PRESSED: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_PRESSED;
        const LV_EVENT_PRESSING: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_PRESSING;
        const LV_EVENT_PRESS_LOST: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_PRESS_LOST;
        const LV_EVENT_SHORT_CLICKED: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_SHORT_CLICKED;
        const LV_EVENT_CLICKED: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_CLICKED;
        const LV_EVENT_LONG_PRESSED: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED;
        const LV_EVENT_LONG_PRESSED_REPEAT: u32 =
            lightvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED_REPEAT;
        const LV_EVENT_RELEASED: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_RELEASED;
        const LV_EVENT_VALUE_CHANGED: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_VALUE_CHANGED;
        const LV_EVENT_DRAW_MAIN: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN;
        const LV_EVENT_DRAW_MAIN_BEGIN: u32 =
            lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_BEGIN;
        const LV_EVENT_DRAW_MAIN_END: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_END;
        const LV_EVENT_DRAW_POST: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST;
        const LV_EVENT_DRAW_POST_BEGIN: u32 =
            lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_BEGIN;
        const LV_EVENT_DRAW_POST_END: u32 = lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_END;

        match value {
            LV_EVENT_PRESSED => Ok(Event::Pressed),
            LV_EVENT_PRESSING => Ok(Event::Pressing),
            LV_EVENT_PRESS_LOST => Ok(Event::PressLost),
            LV_EVENT_SHORT_CLICKED => Ok(Event::ShortClicked),
            LV_EVENT_CLICKED => Ok(Event::Clicked),
            LV_EVENT_LONG_PRESSED => Ok(Event::LongPressed),
            LV_EVENT_LONG_PRESSED_REPEAT => Ok(Event::LongPressedRepeat),
            LV_EVENT_RELEASED => Ok(Event::Released),
            LV_EVENT_VALUE_CHANGED => Ok(Event::ValueChanged),
            LV_EVENT_DRAW_MAIN => Ok(Event::DrawMain),
            LV_EVENT_DRAW_MAIN_BEGIN => Ok(Event::DrawMainBegin),
            LV_EVENT_DRAW_MAIN_END => Ok(Event::DrawMainEnd),
            LV_EVENT_DRAW_POST => Ok(Event::DrawPost),
            LV_EVENT_DRAW_POST_BEGIN => Ok(Event::DrawPostBegin),
            LV_EVENT_DRAW_POST_END => Ok(Event::DrawPostEnd),
            _ => Err(()),
        }
    }
}

impl From<Event> for lightvgl_sys::lv_event_code_t {
    fn from(event: Event) -> Self {
        let native_event = match event {
            Event::Pressed => lightvgl_sys::lv_event_code_t_LV_EVENT_PRESSED,
            Event::Pressing => lightvgl_sys::lv_event_code_t_LV_EVENT_PRESSING,
            Event::PressLost => lightvgl_sys::lv_event_code_t_LV_EVENT_PRESS_LOST,
            Event::ShortClicked => lightvgl_sys::lv_event_code_t_LV_EVENT_SHORT_CLICKED,
            Event::Clicked => lightvgl_sys::lv_event_code_t_LV_EVENT_CLICKED,
            Event::LongPressed => lightvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED,
            Event::LongPressedRepeat => lightvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED_REPEAT,
            Event::Released => lightvgl_sys::lv_event_code_t_LV_EVENT_RELEASED,
            Event::ValueChanged => lightvgl_sys::lv_event_code_t_LV_EVENT_VALUE_CHANGED,
            Event::DrawMain => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN,
            Event::DrawMainBegin => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_BEGIN,
            Event::DrawMainEnd => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_END,
            Event::DrawPost => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST,
            Event::DrawPostBegin => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_BEGIN,
            Event::DrawPostEnd => lightvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_END,
            // TODO: handle all types...
            _ => lightvgl_sys::lv_event_code_t_LV_EVENT_CLICKED,
        };
        native_event as lightvgl_sys::lv_event_code_t
    }
}

pub(crate) fn lv_obj_add_event_cb<'a, F>(widget: &'a mut Wdg, filter: Event, callback: F)
where
    F: FnMut(lightvgl_sys::lv_event_t) + 'a,
{
    unsafe {
        let obj = widget.raw_mut();
        lightvgl_sys::lv_obj_add_event_cb(
            obj,
            Some(event_callback::<F>),
            filter.into(),
            Box::into_raw(Box::new(callback)) as *mut _,
        );
    }
}

pub(crate) unsafe extern "C" fn event_callback<F>(event: *mut lightvgl_sys::lv_event_t)
where
    F: FnMut(lightvgl_sys::lv_event_t),
{
    unsafe {
        let callback = &mut *((*event).user_data as *mut F);
        callback(*event);
    }
}
