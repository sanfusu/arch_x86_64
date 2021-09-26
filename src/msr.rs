use core::marker::PhantomData;

use crate::{
    cpuid::feature::StdFeature,
    mem::segment::{cs::Cs, selector::Privilege},
};

pub mod efer;
pub mod syscfg;

#[derive(Clone, Copy)]
pub struct Msr {
    pub(crate) phatom: PhantomData<usize>,
}
impl Msr {
    pub fn inst(std_feature: &StdFeature) -> Option<Self> {
        // StdFeature 为只读缓存，可以作为入参的判断依据
        if Cs::buffer().selector.rpl() != Privilege::PL0 && !std_feature.support_msr() {
            return None;
        }
        Some(Self {
            phatom: PhantomData,
        })
    }
    pub unsafe fn inst_uncheck() -> Self {
        Self {
            phatom: PhantomData,
        }
    }

    pub fn read(&self, addr: u32) -> u64 {
        unsafe { rdmsr(addr) }
    }
    pub fn write(&mut self, reg_addr: u32, low: u32, high: u32) {
        unsafe { wrmsr(reg_addr, low, high) }
    }
}

/// ## 读 model-specific 寄存器
/// 只能在特权级别为 0 时调用执行，否则会触发通用保护异常（#GP）
#[inline]
unsafe fn rdmsr(addr: u32) -> u64 {
    let mut high: u32;
    let mut low: u32;
    asm!(
        "rdmsr",
        out("edx") high,
        out("eax") low,
        in("ecx") addr,
        options(pure, nomem, nostack),
    );
    (low as u64) | ((high as u64) << 32)
}

/// ## 写 model-specific 寄存器
///
/// 只能在特权级别为 0 时调用执行，否则会触发通用保护异常（#GP）
///
/// 尝试向一个未实现的或保留的 model-specific 寄存器写值，同样会产生 #GP 异常。
#[inline]
unsafe fn wrmsr(reg_addr: u32, low: u32, high: u32) {
    asm!(
        "wrmsr",
        in("edx") high,
        in("eax") low,
        in("ecx") reg_addr,
        options(nomem, nostack),
    );
}
