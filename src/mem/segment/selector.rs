use core::fmt::Display;

use bits::field::BufferReader;

pub struct Selector {
    pub(in crate::mem::segment) data: u16,
}
impl_buffer_trait!(Selector);

/// [`Self::RPL0`]、[`Self::DPL0`]、[`Self::PL0`] 数值上是等价的，也可以互相比较，仅存在语义上的区别。
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Privilege {
    pub(in crate::mem) data: u8,
}
impl Selector {
    pub fn rpl(&self) -> Privilege {
        self.read::<fields::RPL>()
    }
}

impl Display for Privilege {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Privilege::PL0 => f.write_str("PL0"),
            Privilege::PL1 => f.write_str("PL1"),
            Privilege::PL2 => f.write_str("PL2"),
            Privilege::PL3 => f.write_str("PL3"),
            _ => f.write_str("unknown privelege"),
        }
    }
}
#[repr(u8)]
pub enum Privileges {
    PL0,
    PL1,
    PL2,
    PL3,
}
def_const! {
    Privilege {
        pub RPL0: 0,
        pub RPL1: 1,
        pub RPL2: 2,
        pub RPL3: 3,
        pub DPL0: 0,
        pub DPL1: 1,
        pub DPL2: 2,
        pub DPL3: 3,
        pub PL0: 0,
        pub PL1: 1,
        pub PL2: 2,
        pub PL3: 3,
    }
}
pub mod fields {
    use super::{Privilege, Selector};

    bits::fields_ex! {
        Selector [data] {
            /// ### Requestor Privilege-Level Field
            /// 表示选择器被创建时，处理器所处在的权限级别（即 CPL）
            pub RPL [0..=1, rw, Privilege] {
                input_converter: |cpl:Privilege| cpl.data as u16;
                output_converter: |data| Privilege{data: data as u8}
            },
            /// ### Table Indicator
            /// 表示引用的是全局表（GDT），还是本地表（LDT）
            /// + TI = 1 时，LDT
            /// + TI = 0 时，GDT
            pub TI  [2, rw, bool],
            /// ### Selector Index
            /// 用于索引描述符表中的条目
            pub SI  [3..=15, rw, u16]
        }
    }
}
