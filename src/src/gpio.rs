use core::ops;

use register::{mmio::ReadWrite, register_bitfields};

use super::MMIO_BASE;

const GPIO_BASE: u32 = MMIO_BASE + 0x200_000;

// 寄存器文档请参考
// https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf
// TODO
register_bitfields! {
    u32,

    /// GPIO Function Select 1
    GPFSEL1 [
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            RXD0 = 0b100, // UART0     - Alternate function 0
            RXD1 = 0b010  // Mini UART - Alternate function 5

        ],

        /// Pin 14
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            TXD0 = 0b100, // UART0     - Alternate function 0
            TXD1 = 0b010  // Mini UART - Alternate function 5
        ]
    ],

    /// GPIO Pull-up/down Clock Register 0
    GPPUDCLK0 [
        /// Pin 15
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        /// Pin 14
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    pub GPFSEL0: ReadWrite<u32>,
    // 0x00
    pub GPFSEL1: ReadWrite<u32, GPFSEL1::Register>,
    // 0x04
    pub GPFSEL2: ReadWrite<u32>,
    // 0x08
    pub GPFSEL3: ReadWrite<u32>,
    // 0x0C
    pub GPFSEL4: ReadWrite<u32>,
    // 0x10
    pub GPFSEL5: ReadWrite<u32>,
    // 0x14
    __reserved_0: u32,
    // 0x18
    GPSET0: ReadWrite<u32>,
    // 0x1C
    GPSET1: ReadWrite<u32>,
    // 0x20
    __reserved_1: u32,
    //
    GPCLR0: ReadWrite<u32>,
    // 0x28
    __reserved_2: [u32; 2],
    //
    GPLEV0: ReadWrite<u32>,
    // 0x34
    GPLEV1: ReadWrite<u32>,
    // 0x38
    __reserved_3: u32,
    //
    GPEDS0: ReadWrite<u32>,
    // 0x40
    GPEDS1: ReadWrite<u32>,
    // 0x44
    __reserved_4: [u32; 7],
    //
    GPHEN0: ReadWrite<u32>,
    // 0x64
    GPHEN1: ReadWrite<u32>,
    // 0x68
    __reserved_5: [u32; 10],
    //
    pub GPPUD: ReadWrite<u32>,
    // 0x94
    pub GPPUDCLK0: ReadWrite<u32, GPPUDCLK0::Register>,
    // 0x98
    pub GPPUDCLK1: ReadWrite<u32>,                      // 0x9C
}


/// GPIO MMIO 公共接口
pub struct GPIO;

impl ops::Deref for GPIO {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}


impl GPIO {
    pub fn new() -> GPIO {
        GPIO
    }

    /// 返回指向寄存器模块的指针
    fn ptr() -> *const RegisterBlock {
        GPIO_BASE as *const _
    }
}
