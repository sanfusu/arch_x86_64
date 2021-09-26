use core::marker::PhantomData;

use register::RegisterBufferFlush;

use crate::{cpuid::feature::StdFeature, Clean};

use super::Msr;

/// # 扩展特性使能寄存器
/// Extended StdFeature Enable Register
///
/// EFER 是一个 model-specific 寄存器，其地址为 C000_0080h，
/// 只能被特权软件读写。
pub struct Efer {
    msr: Msr,
}

static mut EFER_INSTANCE: Option<Efer> = Some(Efer {
    msr: Msr {
        phatom: PhantomData,
    },
});

impl Drop for Efer {
    fn drop(&mut self) {
        unsafe {
            EFER_INSTANCE.replace(Efer {
                msr: Msr {
                    phatom: PhantomData,
                },
            });
        }
    }
}

impl From<Msr> for Efer {
    #[inline]
    fn from(msr: Msr) -> Self {
        Efer { msr }
    }
}

impl Efer {
    const REG_ADDR: u32 = 0xC000_0080;

    pub fn inst(std_feature: &StdFeature) -> Option<Self> {
        let mut efer = unsafe { EFER_INSTANCE.take()? };
        // Msr::inst 函数中已经检查了特权情况
        let msr = Msr::inst(std_feature)?;
        efer.msr = msr;
        Some(efer)
    }
    pub unsafe fn inst_uncheck() -> Option<Self> {
        EFER_INSTANCE.take()
    }
    #[inline]
    pub fn buffer(&self) -> Option<Clean<EferBuffer>> {
        let mut raw_buffer = unsafe { EFER_BUFFER_INSTANCE.take()? };
        raw_buffer.data = self.msr.read(Self::REG_ADDR) as u32;
        Some(Clean { raw_buffer })
    }
}

pub struct EferBuffer {
    data: u32,
    msr: Msr,
}

const EFER_BUFFER_INSTANCE_DEFAULT: EferBuffer = EferBuffer {
    data: 0,
    msr: Msr {
        phatom: PhantomData,
    },
};

static mut EFER_BUFFER_INSTANCE: Option<EferBuffer> = Some(EFER_BUFFER_INSTANCE_DEFAULT);

impl Drop for EferBuffer {
    fn drop(&mut self) {
        unsafe {
            EFER_BUFFER_INSTANCE.replace(EFER_BUFFER_INSTANCE_DEFAULT);
        }
    }
}

impl RegisterBufferFlush for EferBuffer {
    #[inline]
    fn flush(&mut self) {
        self.msr.write(Efer::REG_ADDR, self.data, 0);
    }
}

impl Clean<EferBuffer> {
    pub fn long_mode_activated(&self) -> bool {
        self.read::<fields::LMA>()
    }
}

impl_reg_buffer_trait!(EferBuffer);

pub mod fields {
    bits::fields_ex! {
        super::EferBuffer [data] {
            INTWB   [18, rw, bool],
            MCOMMIT [17, rw, bool],
            TCE     [15, rw, bool],
            FFXSR   [14, rw, bool],
            LMSLE   [13, rw, bool],
            SVME    [12, rw, bool],
            /// 非执行页保护特性
            NXE     [11, rw, bool],
            /// 用于指示 64 位模式（long mode）是否被激活。
            ///
            /// 注意：该 bit 一般由处理器修改，系统软件虽然可修改，
            /// 但如果值和硬件结果不一致，则会导致 #GP 异常，所以这里认为其是只读位。
            pub(super) LMA     [10, ro, bool],
            /// long mode 使能位（仅仅是有能力激活 long mode），
            /// 只有分页使能后才会真正的激活 long mode。
            ///
            /// 激活 long 模式后，需要将 CS.L 置 1 才能进入到 64-bit 模式。
            LME     [08, rw, bool],
            SCE     [00, rw, bool]
        }
    }
}

#[cfg(test)]
mod test {
    use std::println;

    use crate::msr::efer::{Efer, EferBuffer};

    #[test]
    fn size() {
        println!("{}", core::mem::size_of::<Efer>());
        println!("{}", core::mem::size_of::<EferBuffer>())
    }
}
