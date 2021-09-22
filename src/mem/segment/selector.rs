use core::fmt::Display;

pub struct Selector {
    pub(in crate::mem::segment) data: u16,
}
impl_buffer_trait!(Selector);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Privilege {
    pub(crate) data: u8,
}

impl Display for Privilege {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Privilege { data: 0 } => f.write_str("PL0"),
            Privilege { data: 1 } => f.write_str("PL1"),
            Privilege { data: 2 } => f.write_str("PL2"),
            Privilege { data: 3 } => f.write_str("PL3"),
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
