//! CR2 为页错误虚拟线性地址寄存器。
//! 引发页错误的虚拟线性地址会存放在 CR2 寄存器中，
//! 而错误码则会压入到异常处理程序的栈中。

pub struct Cr2;

impl Cr2 {
    /// 需要确保当前为 legacy 模式
    #[inline]
    pub unsafe fn buffer() -> Cr2Buffer {
        let mut x;
        asm!("mov {}, cr2", out(reg) x);
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
