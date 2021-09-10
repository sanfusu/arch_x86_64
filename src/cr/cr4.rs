use bits::field::{BufferReader, BufferWriter};

pub struct Cr4;
pub struct Cr4Buffer {
    data: usize,
}

impl Cr4 {
    #[inline]
    pub unsafe fn buffer() -> Cr4Buffer {
        let mut x;
        #[cfg(target_arch = "x86")]
        asm!("mov {:e}, cr4", out(reg) x);

        #[cfg(target_arch = "x86_64")]
        asm!("mov {:r}, cr4", out(reg) x);
        Cr4Buffer { data: x }
    }
}
impl Cr4Buffer {
    #[inline]
    pub unsafe fn flush(&mut self) {
        #[cfg(target_arch = "x86")]
        asm!("mov cr4, {:e}", in(reg) self.data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov cr4, {:r}", in(reg) self.data);
    }
}

impl BufferWriter for Cr4Buffer {
    #[must_use = "The modified value works after flushed into register"]
    fn write<T>(&mut self, value: T::ValueType) -> &mut Self
    where
        T: bits::field::Field<Self> + bits::field::FieldWriter<Self>,
    {
        T::write(self, value);
        self
    }
}
impl BufferReader for Cr4Buffer {
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
    bits::fields_ex! {
        super::Cr4Buffer [data] {
            CET         [23, rw, bool],
            PKE         [22, rw, bool],
            SMAP        [21, rw, bool],
            SMEP        [20, rw, bool],
            OSXSAVE     [18, rw, bool],
            PCIDE       [17, rw, bool],
            FSGSBASE    [16, rw, bool],
            UMIP        [11, rw, bool],
            OSXMMEXCPT  [10, rw, bool],
            OSFXSR      [09, rw, bool],
            PCE         [08, rw, bool],
            PGE         [07, rw, bool],
            MCE         [06, rw, bool],
            PAE         [05, rw, bool],
            PSE         [04, rw, bool],
            DE          [03, rw, bool],
            TSD         [02, rw, bool],
            PVI         [01, rw, bool],
            VME         [00, rw, bool]
        }
    }
}
