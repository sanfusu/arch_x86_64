/// Cr4 寄存器除 PCE 位之外，其余 bit 使能前均可以使用 CPUID 指令来判断是否支持该特性。
pub struct Cr4;
pub struct Cr4Buffer {
    data: usize,
}

impl Cr4 {
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

impl_buffer_trait!(Cr4Buffer);

pub mod fields {
    bits::fields_ex! {
        super::Cr4Buffer [data] {
            CET         [23, rw, bool],
            PKE         [22, rw, bool],
            SMAP        [21, rw, bool],
            SMEP        [20, rw, bool],
            OSXSAVE     [18, rw, bool],
            PCIDE       [17, rw, bool],
            FSGSBASE    [16, rw, bool],
            UMIP        [11, rw, bool],
            OSXMMEXCPT  [10, rw, bool],
            OSFXSR      [09, rw, bool],
            PCE         [08, rw, bool],
            PGE         [07, rw, bool],
            MCE         [06, rw, bool],
            PAE         [05, rw, bool],
            PSE         [04, rw, bool],
            DE          [03, rw, bool],
            TSD         [02, rw, bool],
            PVI         [01, rw, bool],
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
            VME         [00, rw, bool]
        }
    }
}
