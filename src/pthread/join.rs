use core::ffi::{c_int, c_void};

use crate::sys::types::pthread_t;

#[unsafe(no_mangle)]
pub extern "C" fn pthread_join(thread: pthread_t, value_ptr: *mut *mut c_void) -> c_int {
    0
}
