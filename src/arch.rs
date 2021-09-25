pub mod extension;

#[cfg(target_arch = "x86_64")]
use crate::cr::cr8::Cr8;

use crate::{
    cpuid::Cpuid,
    cr::{cr0::Cr0, cr2::Cr2, cr3::Cr3, cr4::Cr4},
    mem::segment::{cs::Cs, selector::Privilege},
    msr::efer::Efer,
};

use self::extension::ArchExtension;

// 包含所有可拥有的寄存器实例，这些寄存器实例可用于读写寄存器。
// 虽然包含大量的寄存器实例，但实际上并不占用空间。
pub struct Arch {
    pub cr0: Cr0,
    pub cr2: Cr2,
    pub cr3: Cr3,
    #[cfg(target_arch = "x86_64")]
    pub cr8: Cr8,
}

impl Arch {
    /// 初始化 x86_64 架构
    ///
    /// 该函数需要在 64 位模式下且特权级别为 0 时调用。
    /// （本函数不会进入 64 位模式，进入 64 位模式是保护模式干的事）
    ///
    /// 初始化后，尽管可以安全的读控制寄存器，但并不代表你可以安全的向其中写入值。
    pub fn init() -> Option<Self> {
        if Cs::buffer().selector.rpl() != Privilege::PL0 {
            return None;
        }
        unsafe {
            Some(Self {
                cr0: Cr0::inst_uncheck()?,
                cr2: Cr2::inst_uncheck(),
                cr3: Cr3::inst_uncheck(),
                #[cfg(target_arch = "x86_64")]
                cr8: Cr8::inst_uncheck(),
            })
        }
    }
    pub fn extension(&self) -> Option<ArchExtension> {
        let cpuid = Cpuid::inst()?;
        let std_feature = cpuid.std_feature();
        let efer = Efer::inst(&std_feature)?;
        let cr4 = Cr4::inst()?; // Cr4 中的大部分字段读写均需要参考 std_feature 或通过 Cpuid 来查询

        Some(ArchExtension {
            std_feature,
            cpuid,
            efer,
            cr4,
        })
    }
}
