use bits::field::{BufferReader, BufferWriter};

pub struct Cr3;
impl Cr3 {
    #[inline]
    pub unsafe fn buffer() -> Cr3Buffer {
        let mut x;
        #[cfg(target_arch = "x86")]
        asm!("mov {:e}, cr3", out(reg) x);

        #[cfg(target_arch = "x86_64")]
        asm!("mov {:r}, cr3", out(reg) x);

        Cr3Buffer { data: x }
    }
}

pub struct Cr3Buffer {
    data: usize,
}

impl Cr3Buffer {
    #[inline]
    pub unsafe fn flush(&mut self) {
        #[cfg(target_arch = "x86")]
        asm!("mov cr3, {:e}", in(reg) self.data);

        #[cfg(target_arch = "x86_64")]
        asm!("mov cr3, {:r}", in(reg) self.data);
    }
}
impl BufferWriter for Cr3Buffer {
    #[must_use = "The modified value works after flushed into register"]
    fn write<T>(&mut self, value: T::ValueType) -> &mut Self
    where
        T: bits::field::Field<Self> + bits::field::FieldWriter<Self>,
    {
        T::write(self, value);
        self
    }
}
impl BufferReader for Cr3Buffer {
    fn read<T: bits::field::Field<Self> + bits::field::FieldReader<Self>>(&self) -> T::ValueType {
        T::read(self)
    }

    fn output<T: bits::field::Field<Self> + bits::field::FieldReader<Self>>(
        &self,
        out: &mut T::ValueType,
    ) -> &Self {
        *out = T::read(self);
        self
    }
}

pub mod fields {
    /// # Table Base Address Field
    ///
    /// Legacy 模式下指向最高级别的分页转换表的物理起始地址，该字段的大小依赖于所使用的分页形式：
    ///
    /// + Normal（Non-PAE）`CR4.PAE = 0` 一共 20 bit，占据 31:12 bit，并且指向页目录表的基地址。
    /// 页目录表 4-KB 对其，最低 12 位（11:0）为 0，最低 12 bit 不占据寄存器空间。两者一同组成 32 bit 的基地址。
    ///
    /// + PAE 分页模式 `CR4.PAE = 1`，此时该字段一共 27 bit，占据寄存器的 31:5 bit，指向**页目录指针表**的基地址。
    /// 页目录指针表 32 字节对齐，剩余不占据空间的 5 个 bit 始终为 0。
    ///
    /// Long 模式下用于指向 PML4(Page-Map Level-4) 的基地址（一共 52 bit）。此时 TBA 高 40bit (51:12) 占据寄存器空间；
    /// 低 12bit (11:0) 不占据空间，且始终位 0，也就是说 PML4 表需要 4KB 对齐。
    ///
    /// ❗ Long 模式下写入该字段时，若处理器所支持的物理地址空间小于 52-bit，那么多余的 bit 位需要清 0。
    /// > Amd64.pdf 手册上并没有说明写 1 是否会触发异常。
    pub struct TBA;

    /// PAE 模式下的 TBA
    pub struct TBAPAE;

    /// # Page-Level Cache Disable(PCD) bit
    ///
    /// 页级别的缓存禁用位，用于指示最高级别的页转换表是否缓存。
    ///
    /// + 当 `PCD = 0` 时，转换表可缓存。
    /// + 当 `PCD = 1` 时，转换表不可缓存。
    pub struct PCD;

    /// # Page-level writethrough
    ///
    /// 用于指示最高级别的页转换表是否具有回写或透写的缓存策略。
    ///
    /// + PWT=0，表具有回写缓存策略
    /// + PWT=1，表具有透写缓存策略
    pub struct PWT;

    /// # Process Contex Identifier
    ///
    /// Bits 11:0。`CR4.PCIDE = 1` 时，该 12 bit 字段决定了当前处理器上下文标识符。
    pub struct PCID;

    #[cfg(target_arch = "x86")]
    bits::fields! {
        super::Cr3Buffer [data] {
            TBAPAE [5..=31, rw, usize],
            TBA [12..=31,  rw, usize]
        }
    }

    #[cfg(target_arch = "x86_64")]
    bits::fields! {
        super::Cr3Buffer [data] {
            TBA [12..=51, rw, usize],
            PCID [0..=11, rw, u16]
        }
    }
    bits::fields! {
        super::Cr3Buffer [data] {
            PCD [4, rw, bool],
            PWT [3, rw, bool]
        }
    }
}
