#![no_std]
#![no_main]

/// 很多其他 io 设备基于这个基地址
/// Memory mapping I/O
const MMIO_BASE: u32 = 0x3F00_0000;

fn kernel_entry() -> ! {
    loop {
        let c = '1';

        match c {
            '1' => {}
            '2' => {}
            _ => {}
        }
    }
}

raspi3_boot::entry!(kernel_entry);