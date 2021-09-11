use core::marker::PhantomData;

use bits::field::{BufferReader, BufferWriter};

use crate::cr::flags::{fields, Flags};

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
    /// ⚠️ 由于内部使用了 `popf` 指令，那么在 virtual-8086 模式下，如果 `IOPL` 字段小于 3，且 `CR4.VME` 没有使能，使用本函数则会导致 #GP 异常。
    #[inline]
    pub unsafe fn inst() -> Option<Self> {
        let mut flags = Flags::buffer();
        flags.revert::<fields::ID>().flush();
        if Flags::buffer().read::<fields::ID>() {
            Some(Self {
                phantom: PhantomData,
            })
        } else {
            None
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
