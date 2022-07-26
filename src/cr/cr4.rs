use core::marker::PhantomData;

use register::RegisterBufferFlush;

use crate::{
    mem::segment::{cs::Cs, selector::Privilege},
    Clean, Dirty,
};

/// Cr4 寄存器除 PCE 位之外，其余 bit 使能前均可以使用 CPUID 指令来判断是否支持该特性。
pub struct Cr4 {
    phantom: PhantomData<usize>,
}

static mut CR4_INSTANCE: Option<Cr4> = Some(Cr4 {
    phantom: PhantomData,
});

impl Drop for Cr4 {
    fn drop(&mut self) {
        unsafe {
            CR4_INSTANCE.replace(Cr4 {
                phantom: PhantomData,
            });
        }
    }
}
impl Cr4 {
    pub unsafe fn inst_uncheck() -> Option<Self> {
        CR4_INSTANCE.take()
    }
    pub fn inst() -> Option<Self> {
        if Cs::buffer().selector.rpl() != Privilege::PL0 {
            return None;
        }
        unsafe { CR4_INSTANCE.take() }
    }
    #[inline]
    pub fn buffer(&self) -> Option<Clean<Cr4Buffer>> {
        let mut raw_buffer = unsafe { CR4_BUFFER_INSTANCE.take()? };
        unsafe {
            // 指令执行的安全性以由 inst 实例化检查
            asm!("mov {}, cr4", out(reg) raw_buffer.data);
        }
        Some(Clean { raw_buffer })
    }
}

pub struct Cr4Buffer {
    data: usize,
}

static mut CR4_BUFFER_INSTANCE: Option<Cr4Buffer> = Some(Cr4Buffer { data: 0 });

impl Drop for Cr4Buffer {
    fn drop(&mut self) {
        unsafe {
            CR4_BUFFER_INSTANCE.replace(Cr4Buffer { data: 0 });
        }
    }
}
impl RegisterBufferFlush for Cr4Buffer {
    fn flush(&mut self) {
        unsafe {
            asm!("mov cr4, {}", in(reg) self.data);
        }
    }
}
impl Clean<Cr4Buffer> {
    /// 首先要确保支持 pcid 特性，否则查询结果并不可靠。
    pub unsafe fn pcid_enabled(&self) -> bool {
        self.read::<fields::PCIDE>()
    }
}
impl Dirty<Cr4Buffer> {
    /// 要使能 pcid 必须满足下面两个条件：
    ///
    /// 1. EFER 寄存器中的 LMA 使能，即长模式已经被激活，
    ///     否则返回错误 `ArchError::LongModeInactivated`
    /// 2. 处理器支持 PCID 特性，即 `CPUID.Fn0x1_ecx[17] = 1`，
    ///     否则返回错误：`ArchError::PcidIsNotSupported`
    ///
    /// 这两个条件会在函数内部通过传入的参数 efer_buffer 和 std_feature 来检查。
    pub unsafe fn enable_pcid_uncheck(self) -> Self {
        self.write::<fields::PCIDE>(true)
    }
    pub unsafe fn disable_pcid_uncheck(self) -> Self {
        self.write::<fields::PCIDE>(false)
    }
}

impl_reg_buffer_trait!(Cr4Buffer);

pub mod fields {
    bits::fields_ex! {
        super::Cr4Buffer [data] {
            pub CET         [23, rw, bool],
            pub PKE         [22, rw, bool],
            pub SMAP        [21, rw, bool],
            pub SMEP        [20, rw, bool],
            pub OSXSAVE     [18, rw, bool],
            pub(crate) PCIDE[17, rw, bool],
            pub FSGSBASE    [16, rw, bool],
            pub UMIP        [11, rw, bool],
            pub OSXMMEXCPT  [10, rw, bool],
            pub OSFXSR      [09, rw, bool],
            pub PCE         [08, rw, bool],
            pub PGE         [07, rw, bool],
            pub MCE         [06, rw, bool],
            pub PAE         [05, rw, bool],
            pub PSE         [04, rw, bool],
            /// # 调试扩展位
            /// Debugging Extensions
            ///
            /// + 置 1：使能 I/O 断点，并且将 DR4 和 DR5 置为保留寄存器；此时访问这两个寄存器会导致无效操作码 #UD 异常。
            pub DE          [03, rw, bool],
            /// # 时间戳禁用位
            /// Time-Stamp Disable
            ///
            /// 用来允许控制哪一个特权级别的软件可以读取时间戳计数器；
            ///
            /// + 清 0：任何特权级别的软件均可以使用 RDTSC 或 RDTSCP 指令；
            /// + 置 1：只有特权级别为 0 的软件可以执行上面的两条指令。
            pub TSD         [02, rw, bool],
            /// # 保护模式虚拟中断
            /// Protected-Mode Virtual Interrupts
            ///
            /// + 置 1 使能保护模式虚拟中断。RFLAGS 寄存器中的 VIF 和 VIP 也会被同时使能（如果支持的话）。
            /// + 清 0 禁用。
            ///
            /// 只有 STI 和 CLI 指令会受到 PVI bit 位影响，这两个指令用于使能和禁用 RFLAGS.IF 位。
            pub PVI         [01, rw, bool],
            /// #  virtual-8086 模式扩展
            /// Virtual-8086 Mode Extensions
            ///
            /// + 置 1 后，可以为运行在 virtual-8086 模式下的软件提供由硬件支撑的性能提升。
            /// + 清 0 后，会禁用该提升。
            ///
            /// 具体提升包括：
            ///
            /// 1. 虚拟的、可屏蔽的、外部中断控制（Flags 寄存器中的 VIF、VIP 标签），以及几个可以操控 Flags.IF 位的指令。
            /// 2. 通过使用 TSS 中的中断重定向 bitmap 来选择性的截断软件中断（INTn 指令）。
            pub VME         [00, rw, bool]
        }
    }
}
