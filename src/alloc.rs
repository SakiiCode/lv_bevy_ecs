//! Set LVGL allocator as Rust's global memory allocator

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

// Register the global allocator
#[global_allocator]
static ALLOCATOR: LvglAlloc = LvglAlloc;

/// LVGL allocator. Enabled by toggling the `lvgl-alloc` feature.
pub struct LvglAlloc;

unsafe impl GlobalAlloc for LvglAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            const USIZE_BYTES: usize = (usize::BITS / u8::BITS) as usize;
            let extra_bytes = USIZE_BYTES + (layout.align() - 1);
            let raw = lightvgl_sys::lv_malloc(layout.size() + extra_bytes).cast::<u8>();
            let raw_shifted = raw.add(USIZE_BYTES);
            let offset = raw_shifted.align_offset(layout.align());
            let aligned = raw_shifted.wrapping_add(offset);
            *((aligned.cast::<usize>()).sub(1)) = offset + USIZE_BYTES;
            aligned
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            let offset = *(ptr.cast::<usize>()).sub(1);
            let raw = ptr.sub(offset);
            lightvgl_sys::lv_free(raw.cast::<c_void>())
        }
    }
}
