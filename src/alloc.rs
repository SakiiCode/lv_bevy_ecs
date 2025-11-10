//! Set LVGL allocator as Rust's global memory allocator

use core::alloc::{GlobalAlloc, Layout};

// Register the global allocator
#[global_allocator]
static ALLOCATOR: LvglAlloc = LvglAlloc;

const fn align_up(num: usize, align: usize) -> usize {
    ((num) + ((align) - 1)) & !((align) - 1)
}

/// LVGL allocator. Enabled by toggling the `lvgl-alloc` feature.
pub struct LvglAlloc;

unsafe impl GlobalAlloc for LvglAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            const PTR_OFFSET_SZ: usize = (usize::BITS / u8::BITS) as usize;
            let hdr_size = PTR_OFFSET_SZ + (layout.align() - 1);
            let raw = lightvgl_sys::lv_malloc(layout.size() + hdr_size) as *mut u8;
            let aligned = align_up(raw.add(PTR_OFFSET_SZ) as usize, layout.align()) as *mut u8;
            *(aligned.clone().sub(PTR_OFFSET_SZ) as *mut usize) = aligned.offset_from(raw) as usize;
            aligned
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            let offset = *(ptr as *mut usize).sub(1);
            let raw = ptr.sub(offset);
            lightvgl_sys::lv_free(raw as *mut core::ffi::c_void)
        }
    }
}
