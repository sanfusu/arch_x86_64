//! CR2 为页错误虚拟线性地址寄存器。
//! 引发页错误的虚拟线性地址会存放在 CR2 寄存器中，
//! 而错误码则会压入到异常处理程序的栈中。

use bits::field::{BufferReader, BufferWriter};

pub struct Cr2;

impl Cr2 {
    /// 需要确保当前为 legacy 模式
    #[inline]
    pub unsafe fn buffer() -> Cr2Buffer {
        let mut x;
        #[cfg(target_arch = "x86")]
        asm!("mov {:e}, cr2", out(reg) x);

        #[cfg(target_arch = "x86_64")]
        asm!("mov {:r}, cr2", out(reg) x);
        Cr2Buffer { data: x }
    }
}

#[repr(C)]
pub struct Cr2Buffer {
    data: usize,
}
impl Cr2Buffer {
    pub unsafe fn flush(&mut self) {
        #[cfg(target_arch = "x86")]
        asm!("mov cr2, {:e}", in(reg) self.data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov cr2, {:r}", in(reg) self.data);
    }
}
impl BufferWriter for Cr2Buffer {
    #[must_use = "The modified value works after flushed into register"]
    fn write<T>(&mut self, value: T::ValueType) -> &mut Self
    where
        T: bits::field::Field<Self> + bits::field::FieldWriter<Self>,
    {
        T::write(self, value);
        self
    }
}
impl BufferReader for Cr2Buffer {
    fn read<T: bits::field::Field<Self> + bits::field::FieldReader<Self>>(&self) -> T::ValueType {
        T::read(self)
    }

    fn output<T: bits::field::Field<Self> + bits::field::FieldReader<Self>>(
        &self,
        out: &mut T::ValueType,
    ) -> &Self {
        *out = T::read(self);
        self
    }
}

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
