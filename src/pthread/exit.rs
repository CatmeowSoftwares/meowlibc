use core::ffi::c_void;

#[unsafe(no_mangle)]
pub extern "C" fn pthread_exit(value_ptr: *mut c_void) {}
