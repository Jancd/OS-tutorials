#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

//! Low-level boot of the Raspberry's processor

extern crate panic_abort;

use cortex_a::{asm, regs::*};

/// 处理器启动入口
///
/// 只启动第一个核心 core0, 并且启动成功后执行复位函数：`reset()`
#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn _boot_cores() -> ! {
    const CORE_0: u64 = 0;
    const CORE_MASK: u64 = 0x3;
    const STACK_START: u64 = 0x80_000;

    if CORE_0 == MPIDR_EL1.get() & CORE_MASK {
        SP.set(STACK_START);
        reset()
    } else {
        // if not core0, infinitely wait for events
        loop {
            asm::wfe();
        }
    }
}


/// 复位函数
///
/// 在执行用户的 `main()` 函数之前，初始化 bss 段
unsafe fn reset() -> ! {
    extern "C" {
        // .bss 段的起止, 由链接器脚本提供
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }

    // .bss 段置零
    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    extern "Rust" {
        fn main() -> !;
    }

    main();
}

/// 用户提供的入口函数（`main（）`）的类型检查
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            let f: fn() -> ! = $path;

            f()
        }
    };
}
