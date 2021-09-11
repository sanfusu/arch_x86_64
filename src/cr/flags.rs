use bits::field::{BufferReader, BufferWriter};

pub struct Flags;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct FlagsBuffer {
    data: usize,
}

impl Flags {
    /// 1. 你无法通过本函数来读取 VM 和 RF 标签
    /// 2. ⚠️ virtual-8086 模式下，如果 IOPL 字段小于 3，且 Cr4.VME 没有使能，使用本函数则会导致 #GP 异常。
    #[inline]
    pub unsafe fn buffer() -> FlagsBuffer {
        let mut data;
        asm!(
            "pushf",
            "pop {}",
            out(reg)data,
            options(nomem)
        );
        FlagsBuffer { data }
    }
    /// 置 rFlags 的中断位(IF) 为 0
    ///
    /// 1. 由 INTR 输入的外部中断会被屏蔽
    /// 2. NMI（不可屏蔽中断）不受影响。
    ///
    /// 实模式下，会直接清 0。
    ///
    /// 保护模式和 virtual-8086 模式下，如果 CPL 小于或等于 rFlags.IOPL，则清 0；
    ///
    /// 保护模式下，如果 IOPL < 3，CPL = 3 ，并且保护模式虚拟中断被使能（CR4.PVI），本函数会导致 rFlags.VIF 清 0（IF 不受影响）。
    /// 如果这些条件不满足，则会导致 #GP 异常。
    ///
    ///  virtual-8086 模式下，如果 IOPL < 3 ，并且 virtual-8086 模式扩展使能（CR4.VME=1），本函数会导致 rFlags.VIF 清 0（IF 不受影响）
    #[inline]
    pub unsafe fn disable_if() {
        asm!("cli", options(nomem));
    }
    /// rFlags.IF 置 1
    ///
    /// 这将允许接收 INTR 输入的外部中断。NMI 输入的中断不受影响；
    ///
    /// 实模式下，会直接置 1。
    ///
    /// 保护模式和 virtual-8086 模式下，如果 CPL 小于或等于 rFlags.IOPL，则置 1；
    ///
    /// 保护模式下，如果 IOPL < 3，CPL = 3 ，并且保护模式虚拟中断被使能（CR4.PVI），本函数会导致 rFlags.VIF 置 1（IF 不受影响）。
    /// 如果这些条件不满足，则会导致 #GP 异常。
    ///
    ///  virtual-8086 模式下，如果 IOPL < 3 ，并且 virtual-8086 模式扩展使能（CR4.VME=1），本函数会导致 rFlags.VIF 置 1（IF 不受影响）
    #[inline]
    pub unsafe fn enable_if() {
        asm!("sti", options(nomem))
    }
}
impl FlagsBuffer {
    /// 1. 你无法通过本函数修改 VIP、VIF、VM 标签。
    /// 2. ⚠️  virtual-8086 模式下，如果 IOPL 字段小于 3，且 VME 没有使能，使用本函数则会导致 #GP 异常。
    #[inline]
    pub unsafe fn flush(&mut self) {
        asm!(
            "push {}",
            "popf",
            in(reg) self.data,
            options(nomem)
        )
    }
}
impl BufferReader for FlagsBuffer {}
impl BufferWriter for FlagsBuffer {
    fn write<T>(&mut self, value: T::ValueType) -> &mut Self
    where
        T: bits::field::Field<Self> + bits::field::FieldWriter<Self>,
    {
        T::write(self, value);
        self
    }
}

pub mod fields {
    bits::fields_ex! {
        super::FlagsBuffer [data] {
            /// ## 处理器特性标识位（bit 21）
            /// processor feature identification bit
            ///
            /// 如果软件能够修改该 bit 位，则表示处理器支持 CPUID 指令。
            ID  [21, rw, bool],
            // VIP [20, rw, bool],
            // VIF [19, rw, bool],
            AC  [18, rw, bool],
            // VM  [17, rw, bool],
            // RF  [16, rw, bool],
            NT  [14, rw, bool],
            /// IOPL 字段指定了需要执行 I/O 地址空间指令的特权级别。
            IOPL[12..=13, rw, u8],
            /// 任何特权级别均可读
            OF  [11, rw, bool],
            /// 任何特权级别均可读
            DF  [10, rw, bool],
            IF  [09, rw, bool],
            TF  [08, rw, bool],
            /// 任何特权级别均可读
            SF  [07, rw, bool],
            /// 任何特权级别均可读
            ZF  [06, rw, bool],
            /// 任何特权级别均可读
            AF  [04, rw, bool],
            /// 任何特权级别均可读
            PF  [02, rw, bool],
            /// 任何特权级别均可读
            CF  [00, rw, bool]
        }
    }
}
