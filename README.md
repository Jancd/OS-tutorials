# OS-tutorials

📚 使用 Rust 和树莓派 3 进行裸机与操作系统开发教程。本仓库基于[rust-raspi3-OS-tutorials](https://github.com/rust-embedded/rust-raspi3-OS-tutorials)。

>本教程是 [斯坦福操作系统教程公开课](https://cs140e.sergio.bz/) 的一个示例，本仓库记录📝尽量做到每章节都图文并茂。

## 开发环境与准备

因为是裸机以及操作系统开发会涉及到硬件，所以还需要童鞋们对硬件有一点了解。

### Rust
首先你需要一个合适的 rust 工具链，你需要安装 nightly 版本的 rust 以及 llvm 等等：

```bash
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
$ rustup component add rust-src llvm-tools-preview
$ cargo install cargo-xbuild cargo-binutils
$ rustup component add clippy-preview --toolchain=nightly
```

### Micro SD卡相关

这些在大天朝某宝上都能买到，实用兼实惠。

1. 此外，你需要一个 micro SD 卡，将它格式化为 FAT 格式，并装有树莓派官方提供的 [固件文件](https://github.com/raspberrypi/firmware/tree/master/boot)。
2. 读卡器，就是为 micro SD 卡服务的。

### Usb 转 ttl 串口线

英文翻译过来是 USB 串行调试线，不知道买什么样的同学可以直接某宝搜索“树莓派USB串口线”。

#### 连接方式

电源线🔌与 5v连接，GND 互相连接，TX (GPIO14)、RX (GPIO15)交错连接。有关树莓派的 GPIO 更多信息，请看[官方描述](https://www.raspberrypi.org/documentation/usage/gpio/).



![GPIO](./img/gpio_info.png)

连接参考：
![调试线连接](./img/)

连接上你的电脑后（*nix系统，window的童鞋可以使用串口连接工具）,执行：

```bash
sudo screen /dev/ttyUSB0 115200
```

这样你就可以用过串口线与 pi 通信了，如需退出，可以按 `ctrl-a`, `ctrl-d`。