use core::marker::PhantomData;

pub struct Cr8 {
    phantom: PhantomData<usize>,
}

impl Cr8 {
    pub(crate) unsafe fn inst_uncheck() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
    #[inline]
    pub unsafe fn buffer() -> Cr8Buffer {
        let mut x;
        asm!("mov {}, cr8", out(reg) x);
        Cr8Buffer { data: x }
    }
}

pub struct Cr8Buffer {
    data: usize,
}

/// 用于缓存 CR8 控制寄存器；
impl Cr8Buffer {
    #[inline]
    pub unsafe fn flush(&mut self) {
        asm!("mov cr8, {}", in(reg) self.data);
    }
}
impl_reg_buffer_trait!(Cr8Buffer);

pub mod fields {
    bits::fields_ex! {
        super::Cr8Buffer [data] {
            pub TRR [0..=3, rw, u8]
        }
    }
}
