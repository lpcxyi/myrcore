#![no_std]
#![no_main]
#![feature(global_asm)]

mod lang_items;

const SYSCALL_EXIT: usize = 93;

// syscall and sys_exit 实现退出功能
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe{
        llvm_asm!("ecall"
                  : "={x10}" (ret)
                  : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
                  : "memory"
                  : "volatile"
            );
    }
    ret
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

// 下面定制println
use core::fmt::{self, Write};

const SYSCALL_WRITE: usize = 64;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize{
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

struct Stdout;

impl Write for Stdout{
    fn write_str(&mut self, s: &str) -> fmt::Result{
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments){
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(,$($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(,$($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

// 实现关机功能
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize{
    let mut ret;
    unsafe{
        llvm_asm!("ecall"
                  : "={x10}" (ret)
                  : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
                  : "memory"
                  : "volatile"
            );
    }
    ret
}

const SBI_SHUTDOWN: usize = 8;

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}

// 设置栈
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn _start(){
    // 程序入口函数
//    println!("Hello, world");
    shutdown();
//    sys_exit(9);
}
