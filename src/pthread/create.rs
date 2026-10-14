use core::ffi::{c_int, c_void};

use crate::sys::types::{pthread_attr_t, pthread_t};

#[unsafe(no_mangle)]
pub extern "C" fn pthread_create(
    pthread: *mut pthread_t,
    attr: *const pthread_attr_t,
    start_routine: *const c_void,
    arg: *const c_void,
) -> c_int {
    0
}
