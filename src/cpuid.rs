use core::marker::PhantomData;

#[derive(Debug, Default)]
pub struct CpuidResult {
    pub eax: usize,
    pub ebx: usize,
    pub ecx: usize,
    pub edx: usize,
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
            use register::{RegisterBufferReader, RegisterBufferWriter};

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
    pub fn query(&self, leaf: usize, sub_leaf: usize) -> CpuidResult {
        let mut ret = CpuidResult {
            ..Default::default()
        };
        // 有效值只占用低 32bit，但是在 64bit 模式下会发生 0 扩展，
        // 为了防止编译器利用 r{a,b,c,d}x 寄存器的高 32 bit，这里将结果和入参定义为 usize 类型。
        #[cfg(target_arch = "x86_64")]
        unsafe {
            asm!(
                "mov rbx, {0}", // ebx 是 llvm 内部保留寄存器，无法用作内联汇编的操作数。
                "cpuid",
                "xchg rbx, {0}",
                lateout(reg) ret.ebx,
                inlateout("rax") leaf => ret.eax,
                inlateout("rcx") sub_leaf => ret.ecx,
                lateout("rdx") ret.edx,
                options(nostack, preserves_flags),
            );
        }
        #[cfg(target_arch = "x86")]
        unsafe {
            asm!(
                "mov ebx, {0}", // ebx 是 llvm 内部保留寄存器，无法用作内联汇编的操作数。
                "cpuid",
                "xchg ebx, {0}",
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
