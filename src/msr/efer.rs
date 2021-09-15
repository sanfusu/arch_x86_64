use super::{rdmsr, wrmsr};

/// # 扩展特性使能寄存器
/// Extended Feature Enable Register
///
/// EFER 是一个 model-specific 寄存器，其地址为 C000_0080h，
/// 只能被特权软件读写。
pub struct EFER;
impl EFER {
    const REG_ADDR: u32 = 0xC000_0080;

    #[inline]
    pub unsafe fn buffer() -> EferBuffer {
        EferBuffer {
            data: rdmsr(Self::REG_ADDR) as u32,
        }
    }
}
pub struct EferBuffer {
    data: u32,
}
impl EferBuffer {
    #[inline]
    pub unsafe fn flush(&mut self) {
        wrmsr(EFER::REG_ADDR, self.data, 0);
    }
}

impl_reg_buffer_trait!(EferBuffer);

pub mod fields {
    bits::fields_ex! {
        super::EferBuffer [data] {
            INTWB   [18, rw, bool],
            MCOMMIT [17, rw, bool],
            TCE     [15, rw, bool],
            FFXSR   [14, rw, bool],
            LMSLE   [13, rw, bool],
            SVME    [12, rw, bool],
            NXE     [11, rw, bool],
            /// 用于指示 64 位模式（long mode）是否被激活。
            ///
            /// 注意：该 bit 一般由处理器修改，系统软件虽然可修改，
            /// 但如果值和硬件结果不一致，则会导致 #GP 异常，所以这里认为其是只读位。
            LMA     [10, ro, bool],
            /// long mode 使能位（仅仅是有能力激活 long mode），
            /// 只有分页使能后才会真正的激活 long mode。
            ///
            /// 激活 long 模式后，需要将 CS.L 置 1 才能进入到 64-bit 模式。
            LME     [08, rw, bool],
            SCE     [00, rw, bool]
        }
    }
}
