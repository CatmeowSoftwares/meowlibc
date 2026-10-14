#![no_std]
use core::arch::{asm, global_asm};

pub mod pthread;
pub mod stdbool;
pub mod stdio;
pub mod sys;
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("{}", info.message());
    exit();
    loop {}
}
fn exit() {
    unsafe { asm!("syscall", in("rax") 60, in("rdi") 0) };
}
extern "C" fn init() {}
global_asm!(
    "
    .globl _start
_start:
    call {init}
    xor ebp, ebp
    and rsp, -16
    call main
    mov edi, eax
    mov eax, 60
    syscall
", init = sym init
);
