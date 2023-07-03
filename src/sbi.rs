// 内核与 RustSBI 通信的相关功能
#![allow(unused)]
use core::arch::asm;

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;

// system reset extension
const SRST_EXTENSION: usize = 0x53525354;
const SBI_SHUTDOWN: usize = 0;

// 用于提示编译器对函数进行内联展开，即将函数的代码插入到调用该函数的位置
// which 表示请求RustSBI的服务类型
#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
          "ecall",
          inlateout("x10") arg0 => ret,
          in("x11") arg1,
          in("x12") arg2,
          in("x16") fid,
          in("x17") eid,
        );
    }
    ret
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, c, 0, 0);
}

pub fn shutdown() -> ! {
    sbi_call(SRST_EXTENSION, SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}
