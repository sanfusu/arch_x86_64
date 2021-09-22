use core::fmt::Display;

pub struct Selector {
    pub(in crate::mem::segment) data: u16,
}
impl_buffer_trait!(Selector);

/// [`Self::RPL0`]、[`Self::DPL0`]、[`Self::PL0`] 数值上是等价的，也可以互相比较，仅存在语义上的区别。
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Privilege {
    pub(in crate::mem) data: u8,
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

def_const! {
    Privilege {
        RPL0: 0,
        RPL1: 1,
        RPL2: 2,
        RPL3: 3,
        DPL0: 0,
        DPL1: 1,
        DPL2: 2,
        DPL3: 3,
        PL0: 0,
        PL1: 1,
        PL2: 2,
        PL3: 3,
    }
}
pub mod fields {
    use super::{Privilege, Selector};

    bits::fields_ex! {
        Selector [data] {
            /// Requestor Privilege-Level Field
            /// 表示选择器被创建时，处理器所处在的权限级别（即 CPL）
            pub RPL [0..=1, rw, Privilege] {
                input_converter: |cpl:Privilege| cpl.data as u16;
                output_converter: |data| Privilege{data: data as u8}
            },
            pub TI  [2, rw, bool],
            pub SI  [3..=15, rw, u16]
        }
    }
}
