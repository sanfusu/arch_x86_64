#![feature(asm)]
#![no_std]

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

pub mod arch;
pub mod cpuid;
pub mod cr;
pub mod descriptor;
pub mod msr;
