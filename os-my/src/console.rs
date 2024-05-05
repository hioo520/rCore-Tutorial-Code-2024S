use crate::sbi::sys_call;
use core::fmt::{self, Write};
const SYSCALL_WRITE: usize = 64;

struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    sys_call(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
