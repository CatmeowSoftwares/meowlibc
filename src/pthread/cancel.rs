use core::ffi::c_int;

use crate::sys::types::pthread_t;

#[unsafe(no_mangle)]
pub extern "C" fn pthread_cancel(thread: pthread_t) -> c_int {
    0
}
