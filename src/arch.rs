use crate::cr::{cr0::Cr0, cr2::Cr2};

pub struct Arch {
    pub cr0: Cr0,
    pub cr2: Cr2,
}

impl Arch {
    /// 初始化 x86_64 架构
    ///
    /// 该函数需要在特权级别为 0 时调用。
    pub unsafe fn init() -> Self {
        Self {
            cr0: Cr0::inst(),
            cr2: Cr2::inst(),
        }
    }
}
