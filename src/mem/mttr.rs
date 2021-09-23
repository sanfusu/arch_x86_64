pub struct MtrrFix4K;
impl MtrrFix4K {
    pub const LOW_REG_ADDR: u32 = 0x0267;
    pub const TOP_REG_ADDR: u32 = 0x026f;
}

pub struct MtrrCapBuffer {
    data: u64,
}
pub struct MtrrPhysBaseBuffer {
    data: u64,
}
pub struct MtrrPhysMaskBuffer {
    data: u64,
}

pub struct MtrrDefaultTypeBuffer {
    data: u64,
}
pub struct MemType {
    pub(crate) data: u8,
}

def_const! {
    MemType {
        /// ### Uncacheable
        /// 不可缓存
        ///
        /// 1. ❌所有访问不允许缓存
        /// 2. ❌不允许写合并
        /// 3. ❌不允许预测访问
        UC:0x00,
        /// ### Write-Combining
        /// 写合并
        ///
        /// 1. ❌所有访问不允许缓存
        /// 2. 👌允许写合并
        /// 3. 👌允许预测访问
        WC:0x01,
        /// ### Writethrough
        /// 透写
        ///
        /// 1. 👌读时若 cache 缺失，则申请缓存行
        /// 2. ❌写时若 cache 缺失，不申请缓存行
        /// 3. 写命中，更新缓存和主内存。
        WT:0x04,
        /// ### Write-Protect
        /// 写保护
        ///
        /// 1. 👌读时若 cache 缺失，则申请缓存行
        /// 2. ❌写时若 cache 缺失，不申请缓存行
        /// 3. 写命中，主内存并且使缓存行无效化
        WP:0x05,
        /// ### Writeback
        /// 回写
        ///
        /// 1. 👌读时若 cache 缺失，则申请缓存行，并且可以申请到一个共享、独占或修改状态的缓存行。
        /// 2. 👌写时若 cache 缺失，则申请缓存行，并且会申请到一个处于修改状态下的缓存行。
        ///
        /// 修改状态下的缓存行会被回写到内存中。
        WB:0x06
    }
}
pub mod fields {
    use super::{
        MemType, MtrrCapBuffer, MtrrDefaultTypeBuffer, MtrrPhysBaseBuffer, MtrrPhysMaskBuffer,
    };
    pub struct Type;
    bits::fields! {
        MtrrPhysBaseBuffer [data] {
            Type    [0..=7, rw, MemType] {
                input_converter: |x:MemType| {x.data as u64};
                output_converter: |data|{ MemType{data: data as u8}}
            }
        }
        MtrrDefaultTypeBuffer [data] {
            Type    [0..=7, rw, MemType] {
                input_converter: |x:MemType| {x.data as u64};
                output_converter: |data|{ MemType{data: data as u8}}
            }
        }

    }
    bits::fields_ex! {
        MtrrPhysBaseBuffer [data] {
            /// 52bit 物理空间的基地址，至少 4KB 对齐，不保存低 12bit（永远为 0）
            PhysBase    [12..=51, rw, u64],
        }
        MtrrPhysMaskBuffer [data] {
            /// ### 物理地址范围掩码
            ///
            /// 同时和物理基地址、目的物理地址做与运算，如果两个值相等，则目标物理地址落于物理地址范围内。
            /// 和网络掩码类似的道理。
            PhysMask    [12..=51, rw, u64],
            V           [11, rw, bool]
        }
        MtrrDefaultTypeBuffer [data] {
            /// ### MTRR Enable
            ///
            /// MTRR 内存类型使能位
            ///
            /// + 置 1 时，所有的固定范围、可变范围的 MTRR 均被启用
            /// + 清 0 时，所有的固定范围、可变范围的 MTRR 均被禁用，并且内存类型被置为默认的 UC(uncacheable).
            ///
            /// 该 bit 为不影响 RdMem 和 WrMem 字段的操作。
            E   [11, rw, bool],
            /// ### Fixed-Range Enable
            ///
            /// + 当 FE 置 1 时，所有固定范围的 MTRR 均被启用。
            /// + 当 FE 清 0 时，所有固定范围的 MTRR 均被禁用。
            ///
            /// 该 bit 位对可变范围 MTRR 没有影响。
            FE  [10, rw, bool]
        }
        MtrrCapBuffer [data] {
            WC      [10, ro, bool],
            FIX     [08, ro, bool],
            VCNT    [0..=7, ro, bool]
        }
    }
}
