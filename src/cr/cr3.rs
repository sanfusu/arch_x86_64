#[cfg(target_arch = "x86_64")]
mod pcid;
#[cfg(target_arch = "x86_64")]
pub use pcid::Cr3BufferPcid;

use core::marker::PhantomData;

use register::RegisterBufferFlush;

use crate::Clean;

pub struct Cr3 {
    phatom: PhantomData<usize>,
}

static mut CR3_INSTANCE: Option<Cr3> = Some(Cr3 {
    phatom: PhantomData,
});

impl Drop for Cr3 {
    fn drop(&mut self) {
        unsafe {
            CR3_INSTANCE.replace(Cr3 {
                phatom: PhantomData,
            });
        }
    }
}

impl Cr3 {
    pub(crate) unsafe fn inst_uncheck() -> Option<Self> {
        CR3_INSTANCE.take()
    }
    /// + legacy 模式下 Non-Pae 分页下的 CR3 寄存器读
    /// + Long 模式下 `CR4.PCIDE = 0` 时的 CR3 寄存器读
    #[inline]
    pub fn buffer(&self) -> Option<Clean<Cr3Buffer>> {
        let mut raw_buffer = unsafe { CR3_BUFFER_INSTANCE.take()? };
        unsafe {
            asm!("mov {}, cr3", out(reg) raw_buffer.data);
        }
        Some(Clean { raw_buffer })
    }
}

pub struct Cr3Buffer {
    data: usize,
}
impl Cr3Buffer {
    #[inline]
    pub fn flush(&mut self) {
        unsafe {
            asm!("mov cr3, {}", in(reg) self.data);
        }
    }
}

static mut CR3_BUFFER_INSTANCE: Option<Cr3Buffer> = Some(Cr3Buffer { data: 0 });

impl RegisterBufferFlush for Cr3Buffer {
    // @todo: 所有字段修改均可确保安全后，将 fields 中的字段可见域改为 pub(super)，然后移除 unsafe 关键字。
    #[inline]
    fn flush(&mut self) {
        unsafe {
            asm!("mov cr3, {}", in(reg) self.data);
        }
    }
}
impl Drop for Cr3Buffer {
    fn drop(&mut self) {
        unsafe {
            CR3_BUFFER_INSTANCE.replace(Cr3Buffer { data: 0 });
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl Clean<Cr3Buffer> {
    /// 1. 判断处理器是否支持 pcid，
    /// 2. 判断 pcid 是否已经使能，
    #[inline]
    pub unsafe fn into_pcid_uncheck(self) -> Clean<Cr3BufferPcid> {
        Clean {
            raw_buffer: Cr3BufferPcid {
                buffer: self.raw_buffer,
            },
        }
    }
    #[inline]
    pub fn into_pcid(
        self,
        efer_buffer: &Clean<crate::msr::efer::EferBuffer>,
        cr4_buffer: &Clean<super::cr4::Cr4Buffer>,
    ) -> Option<Clean<Cr3BufferPcid>> {
        if efer_buffer.long_mode_activated() && unsafe { cr4_buffer.pcid_enabled() } {
            Some(unsafe { self.into_pcid_uncheck() })
        } else {
            None
        }
    }
}

#[cfg(target_arch = "x86")]
impl Cr3Buffer {
    /// Legacy-Mode PAE 使能时 Cr3 寄存器读。
    pub unsafe fn into_pae(self) -> Cr3BufferPae {
        Cr3BufferPae { data: self.data }
    }
}

#[cfg(target_arch = "x86")]
pub struct Cr3BufferPae {
    data: usize,
}
#[cfg(target_arch = "x86")]
impl Cr3BufferPae {
    #[inline]
    pub fn flush(&mut self) {
        unsafe {
            asm!("mov cr3, {}", in(reg) self.data);
        }
    }
}

impl_reg_buffer_trait! {
    #[cfg(target_arch="x86")]
    Cr3BufferPae;
    Cr3Buffer;
}

pub mod fields {
    /// ## Table Base Address Field
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

    /// ## Page-Level Cache Disable(PCD) bit
    ///
    /// 页级别的缓存禁用位，用于指示最高级别的页转换表是否缓存。
    ///
    /// + 当 `PCD = 0` 时，转换表可缓存。
    /// + 当 `PCD = 1` 时，转换表不可缓存。
    pub struct PCD;

    /// ## Page-level writethrough
    ///
    /// 用于指示最高级别的页转换表是否具有回写或透写的缓存策略。
    ///
    /// + PWT=0，表具有回写缓存策略
    /// + PWT=1，表具有透写缓存策略
    pub struct PWT;

    /// ## Process Contex Identifier
    ///
    /// Bits 11:0。`CR4.PCIDE = 1` 时，该 12 bit 字段决定了当前处理器上下文标识符。
    #[cfg(target_arch = "x86_64")]
    pub(super) struct PCID;

    #[cfg(target_arch = "x86_64")]
    bits::fields! {
        super::Cr3Buffer [data] {
            PCID    [00..=11, rw, u16],
            TBA     [12..=51, rw, usize]
        }
    }
    #[cfg(target_arch = "x86")]
    bits::fields! {
        super::Cr3Buffer [data] {
            TBA     [12..=31,  rw, usize]
        }
    }
    #[cfg(target_arch = "x86")]
    bits::fields! {
        super::Cr3BufferPae [data] {
            TBA [5..=31, rw, usize]
        }
    }
    bits::fields! {
        super::Cr3Buffer [data] {
            PCD [4, rw, bool],
            PWT [3, rw, bool]
        }
    }
}
