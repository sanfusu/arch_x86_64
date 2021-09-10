macro_rules! crx_buffer {
    ($x:ident, $data:ident) => {
        #[cfg(target_arch = "x86")]
        asm!("mov {:e}, $x", out(reg) $data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov {:r}, $x", out(reg) $data);
    };
}
macro_rules! crx_flush {
    ($x:ident, $data:expr) => {
        #[cfg(target_arch = "x86")]
        asm!("mov $x, {:e}", in(reg) $data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov $x, {:r}", in(reg) $data);
    };
}

pub mod cr0;
pub mod cr2;
pub mod cr3;
pub mod cr4;