use core::marker::PhantomData;

/// Cr4 寄存器除 PCE 位之外，其余 bit 使能前均可以使用 CPUID 指令来判断是否支持该特性。
pub struct Cr4 {
    phantom: PhantomData<usize>,
}
pub struct Cr4Buffer {
    data: usize,
}

impl Cr4 {
    pub(crate) unsafe fn inst_uncheck() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
    #[inline]
    pub unsafe fn buffer() -> Cr4Buffer {
        let mut x;
        asm!("mov {}, cr4", out(reg) x);
        Cr4Buffer { data: x }
    }
}
impl Cr4Buffer {
    #[inline]
    pub unsafe fn flush(&mut self) {
        asm!("mov cr4, {}", in(reg) self.data);
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
            pub PCIDE       [17, rw, bool],
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
