use core::{self, arch::asm};

pub fn sys_call(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        );
    }
    ret
}
const SYSCALL_EXIT: usize = 93;
// 系统退出
pub fn exit(xstate: i32) -> isize {
    sys_call(SYSCALL_EXIT, [xstate as usize, 0, 0])
}
const SBI_SHUTDOWN: usize = 8;
// 系统关机
pub fn shutdown() -> ! {
    sys_call(SBI_SHUTDOWN, [0, 0, 0]);
    panic!("It should shutdown!");
}
