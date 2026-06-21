//! Set LVGL allocator as Rust's global memory allocator

use ::core::alloc::{GlobalAlloc, Layout};
use ::core::ffi::c_void;

#[global_allocator]
static ALLOCATOR: LvglAlloc = LvglAlloc;

/// LVGL allocator. Enabled by toggling the `lvgl-alloc` feature.
pub struct LvglAlloc;

const MIN_ALIGN_BYTES: usize = 16;

unsafe impl GlobalAlloc for LvglAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        #[cfg(all(
            LV_USE_STDLIB_MALLOC = "BUILTIN",
            not(feature = "ctor"),
            not(target_os = "none")
        ))]
        unsafe {
            if !lightvgl_sys::lv_is_initialized() {
                lightvgl_sys::lv_init();
            }
        }

        unsafe {
            const USIZE_BYTES: usize = (usize::BITS / u8::BITS) as usize;
            /// `lv_malloc()` always returns addresses aligned to 4/8 bytes depending on target platform
            ///
            /// however, `hashbrown` sometimes requires 16-byte alignment
            ///
            /// so just make everything 16-byte aligned
            const EXTRA_BYTES: usize = USIZE_BYTES + MIN_ALIGN_BYTES;

            let raw = lightvgl_sys::lv_malloc(layout.size() + EXTRA_BYTES).cast::<u8>();
            if raw.is_null() {
                return raw;
            }

            let raw_shifted = raw.add(USIZE_BYTES);
            let offset = raw_shifted.align_offset(MIN_ALIGN_BYTES);
            let aligned = raw_shifted.add(offset);
            *((aligned.cast::<usize>()).sub(1)) = offset + USIZE_BYTES;
            aligned
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            let offset = *ptr.cast::<usize>().sub(1);
            let raw = ptr.sub(offset);
            lightvgl_sys::lv_free(raw.cast::<c_void>());
        }
    }
}
