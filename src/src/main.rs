#![no_std]
#![no_main]

/// 很多其他 io 设备基于这个基地址
/// Memory mapping I/O
const MMIO_BASE: u32 = 0x3F00_0000;

mod delay;
mod gpio;
mod mbox;
mod power;
mod uart;

fn kernel_entry() -> ! {
    let mut mbox = mbox::Mbox::new();
    let gpio = gpio::GPIO::new();
    let uart = uart::Uart::new();
    let power = power::Power::new();

    // 设置串口输出
    match uart.init(&mut mbox, &gpio) {
        Ok(_) => uart.puts("\n uart is in good working order.\n"),
        Err(_) => loop {
            cortex_a::asm::wfe();
        },
    }

    uart.puts("Press a key to continue booting\n");
    uart.getc();
    uart.puts("\nWelcome to the new OS !\n");

    loop {
        uart.puts("*** sergeychang@gmail.com ***\n");
        uart.puts(">>> 1 - power off\n");
        uart.puts(">>> 2 - reset\n");

        let input = uart.getc();
        match input {
            '1' => {
                if power.off(&mut mbox, &gpio).is_err() {
                    uart.puts("MailBox error in Power::off()!\n");
                    uart.puts("Power off failed.\n");
                }
            }
            '2' => {
                power.reset();
            }
            _ => {}
        }
    }
}

raspi3_boot::entry!(kernel_entry);