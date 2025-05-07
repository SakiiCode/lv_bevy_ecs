pub mod animation;
pub mod styles;
pub mod widgets;
pub mod support;
pub mod alloc;

/// Generic LVGL error. All other errors can be coerced into it.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LvError {
    InvalidReference,
    Uninitialized,
    LvOOMemory,
    AlreadyInUse,
}