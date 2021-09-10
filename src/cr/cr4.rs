use bits::field::{BufferReader, BufferWriter};

pub struct Cr4;
pub struct Cr4Buffer {
    data: usize,
}

impl Cr4 {
    pub unsafe fn buffer() -> Cr4Buffer {
        let mut x;
        crx_buffer!(cr4, x);
        Cr4Buffer { data: x }
    }
}
impl Cr4Buffer {
    pub unsafe fn flush(&mut self) {
        crx_flush!(cr4, self.data);
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
    pub struct CET;
    pub struct PKE;
    pub struct SMAP;
    pub struct SMEP;
    pub struct OSXSAVE;
    pub struct PCIDE;
    pub struct FSGSBASE;
    pub struct UMIP;
    pub struct OSXMMEXCPT;
    pub struct OSFXSR;
    pub struct PCE;
    pub struct PGE;
    pub struct MCE;
    pub struct PAE;
    pub struct PSE;
    pub struct DE;
    pub struct TSD;
    pub struct PVI;
    pub struct VME;

    bits::fields! {
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
