use ::core::{alloc::Layout, ffi::c_void};

use ::alloc::alloc::{alloc, dealloc, realloc};

use lightvgl_sys::{lv_mem_monitor_t, lv_result_t, lv_result_t_LV_RESULT_OK};

// void lv_mem_init(void);
// void * lv_malloc_core(size_t size);
// void * lv_realloc_core(void * p, size_t new_size);
// void lv_free_core(void * p);
// void lv_mem_monitor_core(lv_mem_monitor_t * mon_p);
// lv_result_t lv_mem_test_core(void);

#[cfg(not(LV_USE_STDLIB_MALLOC = "CUSTOM"))]
compile_error!("`rust-alloc` feature requires `LV_USE_STDLIB_MALLOC = CUSTOM`");

#[cfg(feature = "lvgl-alloc")]
compile_error!("`rust-alloc` feature must not be used together with `lvgl-alloc`");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lv_mem_init() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lv_malloc_core(size: usize) -> *mut c_void {
    unsafe {
        const USIZE_BYTES: usize = (usize::BITS / u8::BITS) as usize;

        let new_size = size + USIZE_BYTES;
        let layout = Layout::array::<u8>(new_size).unwrap();

        let raw = alloc(layout);
        let raw_shifted = raw.add(USIZE_BYTES);
        *((raw_shifted.cast::<usize>()).sub(1)) = new_size;
        raw_shifted.cast::<c_void>()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lv_realloc_core(ptr: *mut c_void, new_size: usize) -> *mut c_void {
    unsafe {
        const USIZE_BYTES: usize = (usize::BITS / u8::BITS) as usize;
        let ptr = ptr.cast::<u8>();

        if ptr.is_null() {
            return lv_malloc_core(new_size);
        }

        let old_size = *(ptr.cast::<usize>()).sub(1);
        let old_raw = ptr.sub(USIZE_BYTES);
        let old_layout = Layout::array::<u8>(old_size).unwrap();

        let new_size = new_size + USIZE_BYTES;

        let raw = realloc(old_raw, old_layout, new_size);
        let raw_shifted = raw.add(USIZE_BYTES);
        *((raw_shifted.cast::<usize>()).sub(1)) = new_size;
        raw_shifted.cast::<c_void>()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lv_free_core(ptr: *mut c_void) {
    unsafe {
        const USIZE_BYTES: usize = (usize::BITS / u8::BITS) as usize;
        let ptr = ptr.cast::<u8>();

        let old_size = *(ptr.cast::<usize>()).sub(1);
        let old_raw = ptr.sub(USIZE_BYTES);
        let old_layout = Layout::array::<u8>(old_size).unwrap();

        dealloc(old_raw, old_layout);
    }
}

pub trait MemoryStats {
    fn get_memory_stats(monitor: &mut lv_mem_monitor_t);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lv_mem_monitor_core(monitor: *mut lv_mem_monitor_t) {
    unsafe extern "Rust" {
        fn get_memory_stats(monitor: &mut lv_mem_monitor_t);
    }

    unsafe {
        get_memory_stats(monitor.as_mut().unwrap());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lv_mem_test_core() -> lv_result_t {
    lv_result_t_LV_RESULT_OK
}
