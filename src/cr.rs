/// 这里没办法将 cr0 变成 metvariable，因为没办法在字符串里面做替换。
/// 而 stringify 宏生成的不是字符串字面量。
macro_rules! crx_buffer {
    (cr0, $data:ident) => {
        #[cfg(target_arch = "x86")]
        asm!("mov {:e}, cr0", out(reg) $data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov {:r}, cr0", out(reg) $data);
    };
    (cr2, $data:ident) => {
        #[cfg(target_arch = "x86")]
        asm!("mov {:e}, cr2", out(reg) $data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov {:r}, cr2", out(reg) $data);
    };
    (cr3, $data:ident) => {
        #[cfg(target_arch = "x86")]
        asm!("mov {:e}, cr3", out(reg) $data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov {:r}, cr3", out(reg) $data);
    };
    (cr4, $data:ident) => {
        #[cfg(target_arch = "x86")]
        asm!("mov {:e}, cr4", out(reg) $data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov {:r}, cr4", out(reg) $data);
    };
}
macro_rules! crx_flush {
    (cr0, $data:expr) => {
        #[cfg(target_arch = "x86")]
        asm!("mov cr0, {:e}", in(reg) $data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov cr0, {:r}", in(reg) $data);
    };
    (cr2, $data:expr) => {
        #[cfg(target_arch = "x86")]
        asm!("mov cr2, {:e}", in(reg) $data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov cr2, {:r}", in(reg) $data);
    };
    (cr3, $data:expr) => {
        #[cfg(target_arch = "x86")]
        asm!("mov cr3, {:e}", in(reg) $data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov cr3, {:r}", in(reg) $data);
    };
    (cr4, $data:expr) => {
        #[cfg(target_arch = "x86")]
        asm!("mov cr4, {:e}", in(reg) $data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov cr4, {:r}", in(reg) $data);
    };
}

pub mod cr0;
pub mod cr2;
pub mod cr3;
pub mod cr4;
