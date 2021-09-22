#[cfg(target_arch = "x86_64")]
use crate::cr::cr8::Cr8;
use crate::{
    cr::{cr0::Cr0, cr2::Cr2, cr3::Cr3, cr4::Cr4},
    mem::segment::selector::Privilege,
};

// 包含所有可拥有的寄存器实例，这些寄存器实例可用于读写寄存器。
// 虽然包含大量的寄存器实例，但实际上并不占用空间。
pub struct Arch {
    pub cr0: Cr0,
    pub cr2: Cr2,
    pub cr3: Cr3,
    pub cr4: Cr4,
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
    pub unsafe fn init(pl: &Privilege) -> Option<Self> {
        Some(Self {
            cr0: Cr0::inst(pl)?,
            cr2: Cr2::inst(),
            cr3: Cr3::inst(),
            cr4: Cr4::inst(),
            #[cfg(target_arch = "x86_64")]
            cr8: Cr8::inst(),
        })
    }
}

#[cfg(test)]
mod test {
    use core::mem::size_of;
    use std::println;

    use super::Arch;
    pub struct AA {
        pub test: Option<Arch>,
        pub test2: Option<Arch>,
        pub test1: Option<Arch>,
    }
    #[test]
    pub fn size() {
        println!("{}", size_of::<AA>());
    }
}
