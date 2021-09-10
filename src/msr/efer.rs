use bits::field::{BufferReader, BufferWriter};

use super::{rdmsr, wrmsr};

/// # 扩展特性使能寄存器
/// Extended Feature Enable Register
///
/// EFER 是一个 model-specific 寄存器，其地址为 C000_0080h，
/// 只能被特权软件读写。
pub struct EFER;
impl EFER {
    const REG_ADDR: u32 = 0xC000_0080;

    #[inline]
    pub unsafe fn buffer() -> EferBuffer {
        EferBuffer {
            data: rdmsr(Self::REG_ADDR) as u32,
        }
    }
}
pub struct EferBuffer {
    data: u32,
}
impl EferBuffer {
    #[inline]
    pub unsafe fn flush(&mut self) {
        wrmsr(EFER::REG_ADDR, self.data, 0);
    }
}
impl BufferReader for EferBuffer {
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
impl BufferWriter for EferBuffer {
    #[must_use = "EFER 缓冲区数据刷入到 EFER 寄存器中才能生效"]
    fn write<T>(&mut self, value: T::ValueType) -> &mut Self
    where
        T: bits::field::Field<Self> + bits::field::FieldWriter<Self>,
    {
        T::write(self, value);
        self
    }
}

pub mod fields {
    bits::fields_ex! {
        super::EferBuffer [data] {
            INTWB   [18, rw, bool],
            MCOMMIT [17, rw, bool],
            TCE     [15, rw, bool],
            FFXSR   [14, rw, bool],
            LMSLE   [13, rw, bool],
            SVME    [12, rw, bool],
            NXE     [11, rw, bool],
            LMA     [10, rw, bool],
            LME     [08, rw, bool],
            SCE     [00, rw, bool]
        }
    }
}
