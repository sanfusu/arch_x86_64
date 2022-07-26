pub struct Flags;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct FlagsBuffer {
    data: usize,
}

impl Flags {
    /// 和 buffer 类似，但是可以在实模式（虚拟 8086 模式下调用）
    /// 此时 flags 寄存器只有 16 bit。
    #[inline]
    #[cfg(any(target_arch = "x86", doc))]
    pub unsafe fn real_buffer() -> FlagsBuffer {
        let mut data: u16;
        asm!(
            "pushfw",
            "pop {:x}",
            out(reg)data,
            options(nomem)
        );
        FlagsBuffer {
            data: data as usize,
        }
    }
    /// 1. 你无法通过本函数来读取 VM 和 RF 标签
    /// 2. ⚠️ virtual-8086 模式下，如果 IOPL 字段小于 3，且 Cr4.VME 没有使能，使用本函数则会导致 #GP 异常。
    ///
    /// ⚠️ 实模式（包括虚拟 8086）下请调用 [`real_buffer`](Flags::real_buffer)。
    #[inline]
    pub unsafe fn buffer() -> FlagsBuffer {
        let mut data;
        #[cfg(target_arch = "x86")]
        asm!(
            "pushfd",
            "pop {}",
            out(reg)data,
            options(nomem)
        );
        #[cfg(target_arch = "x86_64")]
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
    ///
    /// ⚠️ 实模式（包括虚拟 8086）下请调用 [`real_flush`](FlagsBuffer::real_flush)。
    #[inline]
    pub fn flush(&mut self) {
        unsafe {
            #[cfg(target_arch = "x86")]
            asm!(
                "push {}",
                "popfd",
                in(reg) self.data,
                options(nomem)
            );
            #[cfg(target_arch = "x86_64")]
            asm!(
                "push {}",
                "popf",
                in(reg) self.data,
                options(nomem)
            );
        }
    }
    /// 类似于 [`flush`](FlagsBuffer::flush)，但是可以在实模式下调用，需要注意实模式下 flags 寄存器只有 16 bit
    #[inline]
    #[cfg(any(target_arch = "x86", doc))]
    pub fn real_flush(&mut self) {
        unsafe {
            asm!(
                "push {:x}",
                "popfw",
                in(reg) self.data,
                options(nomem)
            );
        }
    }
}

impl_reg_buffer_trait!(FlagsBuffer);

pub mod fields {
    bits::fields_ex! {
        super::FlagsBuffer [data] {
            /// ## 处理器特性标识位（bit 21）
            /// processor feature identification bit
            ///
            /// 如果软件能够修改该 bit 位，则表示处理器支持 CPUID 指令。
            pub ID  [21, rw, bool],
            // VIP [20, rw, bool],
            /// # 虚拟中断位
            /// Virtual Interrupt(VIF) Bit
            ///
            /// RFLAGS.IF 位的虚拟映像位。
            ///
            /// 当虚拟 8086 模式扩展 (CR4.VME) 使能时
            /// 或者保护模式虚拟中断 CR4.PVI 使能时，且 RFLAGS.IOPL 小于等于 3 时启用。
            ///
            /// 使能后，原本修改 IF 位的指令会转而修改 VIF 位（不再修改 IF 位）；
            pub VIF     [19, ro, bool],
            pub AC  [18, rw, bool],
            // VM  [17, rw, bool],
            // RF  [16, rw, bool],
            pub NT  [14, rw, bool],
            /// IOPL 字段指定了需要执行 I/O 地址空间指令的特权级别。
            pub IOPL[12..=13, rw, u8],
            /// 任何特权级别均可读
            pub OF  [11, rw, bool],
            /// 任何特权级别均可读
            pub DF  [10, rw, bool],
            pub IF  [09, rw, bool],
            pub TF  [08, rw, bool],
            /// 任何特权级别均可读
            pub SF  [07, rw, bool],
            /// 任何特权级别均可读
            pub ZF  [06, rw, bool],
            /// 任何特权级别均可读
            pub AF  [04, rw, bool],
            /// 任何特权级别均可读
            pub PF  [02, rw, bool],
            /// 任何特权级别均可读
            pub CF  [00, rw, bool]
        }
    }
}
