use super::{rdmsr, wrmsr};

pub struct Syscfg;
pub struct SyscfgBuffer {
    data: u32,
}
impl Syscfg {
    const REG_ADDR: u32 = 0xC001_0010;
    pub unsafe fn buffer() -> SyscfgBuffer {
        SyscfgBuffer {
            data: rdmsr(Self::REG_ADDR) as u32,
        }
    }
}
impl SyscfgBuffer {
    pub unsafe fn flush(&mut self) {
        wrmsr(Syscfg::REG_ADDR, self.data, 0)
    }
}

impl_buffer_trait!(SyscfgBuffer);

pub mod fields {
    bits::fields_ex! {
        super::SyscfgBuffer [data] {
            VMPLE   [25, rw, bool],
            SNPE    [24, rw, bool],
            MEME    [23, rw, bool],
            FWB     [22, rw, bool],
            TOM2    [21, rw, bool],
            MVDM    [20, rw, bool],
            MFDM    [19, rw, bool],
            MFDE    [18, rw, bool]
        }
    }
}
