//! Set LVGL allocator as Rust's global memory allocator

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use std::sync::Mutex;

// Register the global allocator
#[global_allocator]
static ALLOCATOR: LvglAlloc = LvglAlloc {
    lock: Mutex::new(()),
};

/// LVGL allocator. Enabled by toggling the `lvgl-alloc` feature.
pub struct LvglAlloc {
    lock: Mutex<()>,
}

unsafe impl GlobalAlloc for LvglAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _unused = self.lock.lock().unwrap();
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
        let _unused = self.lock.lock().unwrap();
        unsafe {
            let offset = *(ptr.cast::<usize>()).sub(1);
            let raw = ptr.sub(offset);
            lightvgl_sys::lv_free(raw.cast::<c_void>());
        }
    }

    /*unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let _unused = self.lock.lock().unwrap();
        unsafe {
            const USIZE_BYTES: usize = (usize::BITS / u8::BITS) as usize;
            let extra_bytes = USIZE_BYTES + (layout.align() - 1);
            let raw = lightvgl_sys::lv_calloc(layout.size() + extra_bytes, 1).cast::<u8>();
            let raw_shifted = raw.add(USIZE_BYTES);
            let offset = raw_shifted.align_offset(layout.align());
            let aligned = raw_shifted.wrapping_add(offset);
            *((aligned.cast::<usize>()).sub(1)) = offset + USIZE_BYTES;
            aligned
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let _unused = self.lock.lock().unwrap();
        unsafe {
            /*const USIZE_BYTES: usize = (usize::BITS / u8::BITS) as usize;
            let extra_bytes = USIZE_BYTES + (layout.align() - 1);
            let raw = lightvgl_sys::lv_malloc(new_size + extra_bytes).cast::<u8>();
            let raw_shifted = raw.add(USIZE_BYTES);
            let offset = raw_shifted.align_offset(layout.align());
            let aligned = raw_shifted.wrapping_add(offset);
            *((aligned.cast::<usize>()).sub(1)) = offset + USIZE_BYTES;
            lightvgl_sys::lv_memcpy(
                aligned.cast::<c_void>(),
                ptr.cast::<c_void>(),
                layout.size(),
            );
            self.dealloc(ptr, layout);
            aligned*/

            let offset = *(ptr.cast::<usize>()).sub(1);
            let old = ptr.sub(offset).cast::<c_void>();

            const USIZE_BYTES: usize = (usize::BITS / u8::BITS) as usize;
            let extra_bytes = USIZE_BYTES + (layout.align() - 1);

            let raw = lightvgl_sys::lv_realloc(old, new_size + extra_bytes).cast::<u8>();
            let raw_shifted = raw.add(USIZE_BYTES);
            let offset = raw_shifted.align_offset(layout.align());
            let aligned = raw_shifted.wrapping_add(offset);
            *((aligned.cast::<usize>()).sub(1)) = offset + USIZE_BYTES;
            aligned
        }
    }*/
}
