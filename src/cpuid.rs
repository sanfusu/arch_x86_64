use core::marker::PhantomData;

#[derive(Debug, Default)]
pub struct CpuidResult {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

/// 通过不同的功能号，提供关于处理器和其能力的信息。
pub struct Cpuid {
    phantom: PhantomData<usize>,
}
impl Cpuid {
    /// 如果不支持 cpuid 指令，则返回 None，否则返回 Some。
    ///
    /// ⚠️ 在实模式和 virtual-8086 模式下无法通过本函数来获取 Cpuid 指令实例，并且可能会导致异常
    /// （函数内使用了 popf 指令，virtual-8086 模式下可能会导致异常，
    /// 详见 [`Flags::buffer`](crate::cr::flags::Flags::buffer)。
    #[inline]
    pub unsafe fn inst() -> Option<Self> {
        // flags 寄存器所有可编程的 bit 位初始值均为 0；
        // 所以只要能置 1，或者其值已经为 1，均表明可以修改。
        #[cfg(target_arch = "x86")]
        {
            use crate::cr::flags::{fields, Flags};
            use bits::field::{BufferReader, BufferWriter};

            let mut flags = Flags::buffer();
            flags.write::<fields::ID>(true).flush();
            if Flags::buffer().read::<fields::ID>() {
                Some(Self {
                    phantom: PhantomData,
                })
            } else {
                None
            }
        }
        // 64 bit CPU 均支持 CPUID 指令。
        #[cfg(target_arch = "x86_64")]
        {
            Some(Self {
                phantom: PhantomData,
            })
        }
    }
    /// 根据功能号查询处理器信息和其特性。
    #[inline]
    pub fn query(&self, leaf: u32, sub_leaf: u32) -> CpuidResult {
        let mut ret = CpuidResult {
            ..Default::default()
        };
        unsafe {
            asm!(
                "mov ebx, {0:e}",
                "cpuid",
                "xchg ebx, {0:e}",
                lateout(reg) ret.ebx,
                inlateout("eax") leaf => ret.eax,
                inlateout("ecx") sub_leaf => ret.ecx,
                lateout("edx") ret.edx,
                options(nostack, preserves_flags),
            );
        }
        ret
    }
}
