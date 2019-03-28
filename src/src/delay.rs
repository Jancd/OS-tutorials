use cortex_a::asm;

/// 等待 N 个机器周期
/// 此函数仅限于 arm cpu
pub fn wait_cycles(cycle: u32) {
    for _ in 0..cycle {
        asm::nop();
    }
}