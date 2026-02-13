//! Set LVGL allocator as Rust's global memory allocator

use ::core::alloc::{GlobalAlloc, Layout};
use ::core::ffi::c_void;
#[cfg(not(feature = "ctor"))]
use ::core::sync::atomic::{AtomicBool, Ordering};

// If ctor has run we can assume heap memory has been reserved
#[cfg(feature = "ctor")]
#[global_allocator]
static ALLOCATOR: LvglAllocUnchecked = LvglAllocUnchecked;

/// LVGL allocator. Enabled by toggling the `lvgl-alloc` feature.
pub struct LvglAllocUnchecked;

unsafe impl GlobalAlloc for LvglAllocUnchecked {
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
            lightvgl_sys::lv_free(raw.cast::<c_void>());
        }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
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
        unsafe {
            let offset = *(ptr.cast::<usize>()).sub(1);
            let old = ptr.sub(offset).cast::<c_void>();

            const USIZE_BYTES: usize = (usize::BITS / u8::BITS) as usize;
            let extra_bytes = USIZE_BYTES + (layout.align() - 1);

            let raw = lightvgl_sys::lv_realloc(old, new_size + extra_bytes).cast::<u8>();
            let raw_shifted = raw.add(USIZE_BYTES);
            let offset = raw_shifted.align_offset(layout.align());
            let aligned = raw_shifted.add(offset);
            *((aligned.cast::<usize>()).sub(1)) = offset + USIZE_BYTES;
            aligned
        }
    }
}

// If ctor is not enabled, we can't be sure if lv_init has been called
#[cfg(not(feature = "ctor"))]
#[global_allocator]
static ALLOCATOR: LvglAllocSafe = LvglAllocSafe(LvglAllocUnchecked);

#[cfg(not(feature = "ctor"))]
static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[cfg(not(feature = "ctor"))]
pub struct LvglAllocSafe(LvglAllocUnchecked);

#[cfg(not(feature = "ctor"))]
unsafe impl GlobalAlloc for LvglAllocSafe {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if !INITIALIZED.load(Ordering::Acquire) {
            log::warn!("LVGL was not initialized! Running lv_init() now");
            crate::functions::lv_init();
            INITIALIZED.store(true, Ordering::Release);
        }
        unsafe { self.0.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { self.0.dealloc(ptr, layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if !INITIALIZED.load(Ordering::Acquire) {
            log::warn!("LVGL was not initialized! Running lv_init() now");
            crate::functions::lv_init();
            INITIALIZED.store(true, Ordering::Release);
        }
        unsafe { self.0.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { self.0.realloc(ptr, layout, new_size) }
    }
}
