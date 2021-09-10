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
            CET [23, rw, bool]
        }
    }
    #[cfg(target_arch = "x86_64")]
    bits::fields! {
        super::Cr4Buffer [data] {
            PKE     [22, rw, bool],
            SMAP    [21, rw, bool]
        }
    }
}
