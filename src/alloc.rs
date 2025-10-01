//! Set LVGL allocator as Rust's global memory allocator

use core::alloc::{GlobalAlloc, Layout};

// Register the global allocator
#[global_allocator]
static ALLOCATOR: LvglAlloc = LvglAlloc;

/// LVGL allocator. Enabled by toggling the `lvgl-alloc` feature.
pub struct LvglAlloc;

unsafe impl GlobalAlloc for LvglAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { lightvgl_sys::lv_malloc(layout.size() as cty::size_t) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { lightvgl_sys::lv_free(ptr as *mut cty::c_void) }
    }
}
