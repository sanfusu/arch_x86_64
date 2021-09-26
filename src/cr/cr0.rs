use core::marker::PhantomData;

use register::RegisterBufferFlush;

use crate::{
    mem::segment::{cs::Cs, selector::Privilege},
    Clean,
};

/// CR0 控制寄存器；
///
/// 基本读取流程：
/// ```not test
/// let mut cr0_buff = Cr0::buff();
/// cr0_buff.write::<fields::PG>(true).flush(); // modify the buff and flush into the register.
/// let is_paging = cr0_buff.read::<fields::PG>(); // read the buff rather than the register.
/// ```
pub struct Cr0 {
    phantom: PhantomData<usize>,
}

static mut CR0_INSTANCE: Option<Cr0> = Some(Cr0 {
    phantom: PhantomData,
});

impl Cr0 {
    pub fn inst() -> Option<Self> {
        if Cs::buffer().selector.rpl() != Privilege::RPL0 {
            return None;
        }
        unsafe { CR0_INSTANCE.take() }
    }
    pub(crate) unsafe fn inst_uncheck() -> Option<Self> {
        CR0_INSTANCE.take()
    }
    #[inline]
    pub fn buffer(&self) -> Option<Clean<Cr0Buffer>> {
        let mut x = unsafe { CR0BUFFER_INSTANCE.take()? };
        unsafe {
            asm!("mov {}, cr0", out(reg) x.data);
        }
        Some(Clean { raw_buffer: x })
    }
}

impl Drop for Cr0 {
    fn drop(&mut self) {
        unsafe {
            CR0_INSTANCE = Some(Cr0 {
                phantom: PhantomData,
            });
        }
    }
}

pub struct Cr0Buffer {
    data: usize,
}

static mut CR0BUFFER_INSTANCE: Option<Cr0Buffer> = Some(Cr0Buffer { data: 0 });

impl Drop for Cr0Buffer {
    fn drop(&mut self) {
        // self.data 中的值是不可信任的，所以这里设为 0。
        // 在次执行 Cr0::buffer 时，会从寄存器中在次读出数值。
        unsafe { CR0BUFFER_INSTANCE = Some(Cr0Buffer { data: 0 }) }
    }
}

impl RegisterBufferFlush for Cr0Buffer {
    fn flush(&mut self) {
        unsafe {
            asm!("mov cr0, {}", in(reg) self.data);
        }
    }
}

impl_reg_buffer_trait!(Cr0Buffer);

pub mod fields {
    bits::fields_ex! {
        super::Cr0Buffer [data] {
            pub PG [31, rw, bool],
            /// # Cache Disable (CD) Bit
            ///
            /// + 当 CD 清 0 时，内部缓存会被启用。
            /// + CD 置 1 时，禁用缓存，数据和指令都不会写入到缓存中，但不影响对已有数据的命中访问。
            /// + CD 置 1 时，处理器会忽视 page 级别的缓存控制位 (PWT 和 PCD)，如果分页使能的话。PWT 和 PCD 位于 CR3 寄存器中。
            ///
            /// 软件可以通过置 CD 为 1，并将 cache 无效化来禁止访问缓存。
            pub CD [30, rw, bool],
            /// # Not Writethrough (NW) Bit
            ///
            /// 清 0 或置 1 均会被忽视，该字段已被废弃。
            pub NW [29, rw, bool],
            /// # Alignment Mask (AM) Bit
            ///
            /// 当 RFLAGS.AC = 1 时，可以通过置 AM 为 1 来使能自动对齐检查。
            /// RFLAGS.AC = 0 或 AM = 0，均可以禁用自动对齐检查。
            /// 当自动对齐检查使能，并且 CPL = 3 时，引用未对齐操作数会导致对齐检查异常 (#AC)
            pub AM [18, rw, bool],
            /// # Write Protect (WP) Bit
            ///
            /// + WP 置 1 时，supervisor-level 的软件无法写入只读页面。
            /// + WP 置 0 时，supervisor-level 的软件可以写入只读页面。
            pub WP [16, rw, bool],
            /// # Numeric Error (NE) Bit
            ///
            /// NE 清 0，会禁用 x87 浮点异常的内部控制，并使能外部控制，
            /// 此时 IGNNE# 输入信号控制是否忽略 x87 浮点异常：
            ///
            /// + IGNNE# 为 1 时，x87 浮点异常被忽略
            /// + IGNNE# 为 0 时，x87 浮点异常通过设置 FERR# 输入信号为 1 来报告异常。
            /// 外部逻辑可以使用 FERR# 信号作为一个外部中断。
            ///
            /// NE 置 1，则使用内部报告机制，并禁用外部报告机制。
            /// 推荐软件置 NE 为 1，这种情况下会有更好的 x87 浮点异常处理性能。
            pub NE [5,  rw, bool],
            /// # Extension Type (ET) Bit
            ///
            /// 只读。部分早期 x86 处理器中，软件会将 ET 置 1 来表明支持 387DX 数学协处理器指令。
            /// 该 bit 现在属于保留字段，并强制为 1，软件无法清零。
            pub ET [4,  ro, bool],
            /// # Task Switched (TS) Bit
            ///
            /// TS 置 1 后：
            ///
            /// + 执行 x87 或媒体指令会触发 device-not-available 异常 (#NM)。
            ///
            /// 软件可以使用该机制延迟上下文切换，从而在执行下一条同类型指令前保存指令单元的上下文。
            /// 相应的，x87 和媒体指令单元的上下文只会在任务切换时保存。
            ///
            /// 当发生硬件任务切换时，TS 自动置 1。如果任务切换由软件实现，则依旧可以使用 TS 位来
            /// 控制 x87 和媒体指令单元的上下文切换。这种情况下，任务管理软件使用 MOV CR0 指令在任务切换时显式的将
            /// TS 位置 1。软件可以通过 CLTS 指令或直接写 CR0 寄存器的方式将 TS 位清 0。
            /// 这种软件控制的方法在长模式下依旧奏效（长模式不支持硬件任务切换机制）。
            ///
            /// 当 TS=1 时 [`MP`] 位控制 WAIT/FWAIT 指令是否会触发 #NM 异常。
            pub TS [3,  rw, bool],
            /// # Emulate Coprocessor (EM) Bit
            ///
            /// EM 置 1 后：
            ///
            /// + 运行 x87 指令会触发 device-not-available 异常 (#NM),
            /// + 运行 64-bit 或 128-bit 媒体指令会导致 invalid-opcode 异常 (#UD)
            ///
            /// 如果有需要，可以在异常处理程序中模拟这些指令，从而达到模拟协处理器的目的。
            ///
            /// ❗ EM 置 1 后，WAIT/FWAIT 指令执行不会导致 #NM 异常。
            pub EM [2,  rw, bool],
            /// # Monitor Coprocessor (MP) Bit
            ///
            /// 和任务切换 [`TS`] 位一起协同控制 `WAIT/FWAIT` 指令执行时是否会触发 device-not-available 异常 (#NM)
            ///
            /// + 如果 MP 和 TS 均置位，执行 `WATI/FWAIT` 指令会触发 device-not-available 异常 (#NM)。
            /// + 如果 MP 和 TS 中任意一位清 0，则 `WAIT/FWAIT` 正常执行。
            ///
            /// 如果处理器支持 x87 指令，则通常将 MP 置 1，以便由 TS 来掌控因上下文切换而导致的 x87 指令上下文环境保存的时机。
            pub MP [1,  rw, bool],
            /// # Protected-Mode Enable (PE) Bit
            ///
            /// 保护模式使能位，写 true 使能，写 false 禁用。
            /// 处理器运行在保护模式时，段保护机制会被使能。
            pub PE [0,  rw, bool]
        }
    }
}

#[cfg(test)]
mod test {

    use crate::cr::cr0::{fields, Cr0};

    #[test]
    #[ignore]
    pub fn cr0_instance() {
        {
            let cr0_buffer_clean = Cr0::inst().unwrap().buffer().unwrap();
            if cr0_buffer_clean.read::<fields::AM>() {
                let cr0_buffer_dirty = cr0_buffer_clean.write::<fields::AM>(false);
                cr0_buffer_dirty.flush();
            }
        }
        let _cr0_ro1 = Cr0::inst().unwrap().buffer().unwrap();
    }
}
