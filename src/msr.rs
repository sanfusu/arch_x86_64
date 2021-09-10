pub mod efer;
pub mod syscfg;

/// ## 读 model-specific 寄存器
/// 只能在特权级别为 0 时调用执行，否则会触发通用保护异常（#GP）
#[inline]
pub unsafe fn rdmsr(addr: u32) -> u64 {
    let mut high: u32;
    let mut low: u32;
    asm!(
        "rdmsr",
        out("edx") high,
        out("eax") low,
        in("ecx") addr,
        options(pure, nomem, nostack),
    );
    (low as u64)| ((high as u64) << 32)
}

/// ## 写 model-specific 寄存器
///
/// 只能在特权级别为 0 时调用执行，否则会触发通用保护异常（#GP）
///
/// 尝试向一个未实现的或保留的 model-specific 寄存器写值，同样会产生 #GP 异常。
#[inline]
pub unsafe fn wrmsr(reg_addr: u32, low: u32, high: u32) {
    asm!(
        "wrmsr",
        in("edx") high,
        in("eax") low,
        in("ecx") reg_addr,
        options(nomem, nostack),
    );
}