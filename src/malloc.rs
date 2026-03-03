use core::ffi::c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    unsafe { lightvgl_sys::lv_malloc(size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn calloc(num: usize, size: usize) -> *mut c_void {
    unsafe { lightvgl_sys::lv_calloc(num, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc(data_p: *mut c_void, size: usize) -> *mut c_void {
    unsafe { lightvgl_sys::lv_realloc(data_p, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(data: *mut c_void) {
    unsafe { lightvgl_sys::lv_free(data) }
}
