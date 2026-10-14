use core::{
    arch::asm,
    ffi::{CStr, c_char, c_int},
    fmt::{Arguments, Write},
};

struct StdOut;

impl Write for StdOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            asm!("
            syscall
        ", in("rax") 1, in("rdi") s.len(), in("rsi") s.as_ptr())
        };
        Ok(())
    }
}
#[unsafe(no_mangle)]
pub fn _print(args: Arguments) {
    StdOut.write_fmt(args).expect("failed to write");
}
#[macro_export]
macro_rules! print {
    () => {};
    ($($arg:tt)*) => {
        $crate::stdio::_print(format_args!($($arg)*))
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn printf(format: *const c_char, mut args: ...) -> c_int {
    let cstr = unsafe { CStr::from_ptr(format) };
    let s = cstr.to_str().expect("failed to convert to string");
    let splits = s.split('%');
    for split in splits {
        let mut chars = split.chars();
        match chars.next() {
            Some('d') => {
                let arg = unsafe { args.next_arg::<i32>() };
                print!("{arg}{}", chars.as_str());
            }
            Some('f') => {
                let arg = unsafe { args.next_arg::<f64>() };
                print!("{arg}{}", chars.as_str());
            }
            Some('c') => {
                let arg = unsafe { args.next_arg::<i32>() };
                print!("{}{}", arg as u8 as char, chars.as_str());
            }
            Some('s') => {
                let arg = unsafe { args.next_arg::<*const c_char>() };
                print!(
                    "{}{}",
                    unsafe { CStr::from_ptr(arg).display() },
                    chars.as_str()
                );
            }
            Some('x') => {
                let arg = unsafe { args.next_arg::<u32>() };
                print!("{arg:x}{}", chars.as_str());
            }
            Some('p') => {
                let arg = unsafe { args.next_arg::<*const ()>() };
                print!("{arg:p}{}", chars.as_str());
            }
            Some('%') => {
                print!("%{}", chars.as_str());
            }
            _ => {
                print!("{split}");
            }
        }
    }
    s.len() as _
}
