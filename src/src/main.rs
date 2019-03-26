#![no_std]
#![no_main]

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