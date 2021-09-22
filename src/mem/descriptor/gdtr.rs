use core::{convert::TryInto, ptr::slice_from_raw_parts};

pub struct GDTR;

#[derive(Debug)]
pub struct GdtrBuffer {
    /// 全局描述符表的界限（大小）。
    /// limit + base_addr 决定了 GDT 的结束字节的地址。
    ///
    /// ⚠️ 如果软件尝试访问超高界限的描述符，则会触发 #GP 异常
    pub limit: u16,
    /// GDT 在虚拟内存空间中起始字节地址。
    ///
    /// GDT 可以位于*虚拟内存*中的任何字节位置，
    /// 但是系统软件也该将其放置在 8 字节对齐的地方，
    /// 以避免出现非对齐访问的性能问题。
    pub base_addr: usize,
}

impl GDTR {
    const LEN: usize = core::mem::size_of::<usize>() + 2;

    #[inline]
    pub unsafe fn buffer() -> GdtrBuffer {
        let mut ptr = [0u8; GDTR::LEN].as_mut_ptr();

        asm!(
            "sgdt [{}]", inout(reg) ptr, options(nostack, preserves_flags)
        );

        let buffer = &*slice_from_raw_parts(ptr, GDTR::LEN);

        // @safety_unwrap_panic:
        // 这里的 unrwap 不会导致 panic，应为长度已经由 LEN 确保了。
        GdtrBuffer {
            limit: u16::from_le_bytes(buffer[0..=1].try_into().unwrap()),
            base_addr: usize::from_le_bytes(buffer[2..].try_into().unwrap()),
        }
    }
}
impl GdtrBuffer {
    /// 只能在 CPL0 时调用。 一般在切换到保护模式前调用。
    #[inline]
    pub unsafe fn flush(&mut self) {
        let mut bytes = [0u8; GDTR::LEN];

        // @safety_implicity_panic:
        // copy_from_slice 的源长和目的长，由 u16 为确定的两字节长度保证。不会导致 panic。
        bytes[0..=1].copy_from_slice(&self.limit.to_le_bytes());

        // @safety_implicity_panic:
        // copy_from_slice 的长度由 GDTR::LEN 保证，不会导致 panic。
        bytes[2..].copy_from_slice(&self.base_addr.to_le_bytes());

        let ptr = bytes.as_mut_ptr();

        asm!(
            "lgdt [{}]", in(reg) ptr, options(nostack, preserves_flags)
        );
    }
}
