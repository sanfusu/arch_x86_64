use core::{convert::TryInto, ptr::slice_from_raw_parts};

pub struct GDTR;

#[derive(Debug)]
pub struct GdtrBuffer {
    pub limit: u16,
    pub base_addr: usize,
}

impl GDTR {
    #[inline]
    pub unsafe fn buffer() -> GdtrBuffer {
        const LEN: usize = core::mem::size_of::<usize>() + 2;

        let mut ptr = [0u8; LEN].as_mut_ptr();

        asm!(
            "sgdt [{}]", inout(reg) ptr, options(nostack)
        );

        let buffer = &*slice_from_raw_parts(ptr, LEN);

        // @unwrap 这里的 unrwap 不会导致 panic，应为长度已经由 LEN 确保了。
        GdtrBuffer {
            limit: u16::from_le_bytes(buffer[0..=1].try_into().unwrap()),
            base_addr: usize::from_le_bytes(buffer[2..].try_into().unwrap()),
        }
    }
}
