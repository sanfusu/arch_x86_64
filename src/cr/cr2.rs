//! CR2 为页错误虚拟线性地址寄存器。
//! 引发页错误的虚拟线性地址会存放在 CR2 寄存器中，
//! 而错误码则会压入到异常处理程序的栈中。

use core::marker::PhantomData;

use crate::mem::segment::{cs::Cs, selector::Privilege};

pub struct Cr2 {
    phantom: PhantomData<usize>,
}

impl Cr2 {
    pub fn inst() -> Option<Self> {
        if Cs::buffer().selector.rpl() == Privilege::PL0 {
            Some(Self {
                phantom: PhantomData,
            })
        } else {
            None
        }
    }
    pub fn inst_uncheck() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn buffer(&self) -> Cr2Buffer {
        let mut x;
        unsafe {
            // 只有 inst 可以创建 Cr2 实例，安全性由 inst 函数确保。
            asm!("mov {}, cr2", out(reg) x);
        }
        Cr2Buffer { data: x }
    }
}

#[repr(C)]
pub struct Cr2Buffer {
    data: usize,
}
impl Cr2Buffer {
    pub unsafe fn flush(&mut self) {
        asm!("mov cr2, {}", in(reg) self.data);
    }
}

impl_reg_buffer_trait!(Cr2Buffer);

pub mod fields {
    /// # Page Fault Virtual Address
    ///
    /// 页错误虚拟地址
    pub struct PFVA;

    #[cfg(target_arch = "x86")]
    bits::fields! {
        super::Cr2Buffer [data] {
            PFVA [0..=31, rw, usize]
        }

    }
    #[cfg(target_arch = "x86_64")]
    bits::fields! {
        super::Cr2Buffer [data] {
            PFVA [0..=63, rw, usize]
        }
    }
}
