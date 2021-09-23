#![feature(asm)]
#![no_std]

#[cfg(test)]
extern crate std;

macro_rules! plain_field {
    ($Struct:path {$($Vis:vis $Fn:ident:$Field:path),* $(,)?}) => {
        impl $Struct {
            $(
                #[inline]
                $Vis fn $Fn(&self)-><$Field as bits::field::Field<$Struct>>::ValueType {
                    use register::RegisterBufferReader;
                    self.read::<$Field>()
                }
            )*
        }
    };
}

macro_rules! impl_buffer_trait {
    ($($(#[$Attr:meta])? $Buffer:ident);+ $(;)?) => {
        $(
            $(#[$Attr])?
            impl bits::field::BufferReader for $Buffer {
            }
            $(#[$Attr])?
            impl bits::field::BufferWriter for $Buffer {
            }
        )+
    };
}

macro_rules! impl_reg_buffer_trait {
    ($($(#[$Attr:meta])? $Buffer:ident);+ $(;)?) => {
        $(
            $(#[$Attr])?
            impl register::RegisterBufferReader for $Buffer {
            }
            $(#[$Attr])?
            impl register::RegisterBufferWriter for $Buffer {
            }
        )+
    };
}

macro_rules! def_const {
    ($(
        $Struct:path {
            $(
                $(#[$Attr:meta])*
                $Vis:vis $CosntName:ident:$ConstValue:literal
            ),+ $(,)?
        }
    )+) => {
        $(
            impl $Struct {
                $(
                    $(#[$Attr])*
                    pub const $CosntName:$Struct = { $Struct {data:$ConstValue} };
                )+
            }
        )+
    };
}
pub mod arch;
pub mod cpuid;
pub mod cr;
pub mod mem;
pub mod msr;

#[derive(Debug)]
pub enum ArchError {
    LongModeInactivated,
    PcidIsNotSupported,
    PcidDisabled,
}
