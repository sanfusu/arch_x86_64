#![feature(asm)]
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering::Relaxed};

use register::{RegisterBufferFlush, RegisterBufferReader, RegisterBufferWriter};

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

pub struct Clean<T: RegisterBufferReader + RegisterBufferWriter + RegisterBufferFlush> {
    pub(crate) raw_buffer: T,
}

impl<T: RegisterBufferReader + RegisterBufferWriter + RegisterBufferFlush> Clean<T> {
    pub fn read<Field: bits::field::Field<T> + bits::field::FieldReader<T>>(
        &self,
    ) -> Field::ValueType {
        Field::read(&self.raw_buffer)
    }
    pub fn output<Field: bits::field::Field<T> + bits::field::FieldReader<T>>(
        &self,
        out: &mut Field::ValueType,
    ) -> &Self {
        *out = Field::read(&self.raw_buffer);
        self
    }

    #[must_use = "The modified value works after flushed into register"]
    pub fn write<Field>(mut self, value: Field::ValueType) -> Dirty<T>
    where
        Field: bits::field::Field<T> + bits::field::FieldWriter<T>,
        Self: Sized,
    {
        Field::write(&mut self.raw_buffer, value);
        Dirty {
            raw_buffer: self.raw_buffer,
        }
    }
    pub fn asume_dirty(self) -> Dirty<T> {
        Dirty {
            raw_buffer: self.raw_buffer,
        }
    }
}

pub struct Dirty<T: RegisterBufferReader + RegisterBufferWriter + RegisterBufferFlush> {
    pub(crate) raw_buffer: T,
}
impl<T: RegisterBufferReader + RegisterBufferWriter + RegisterBufferFlush> Dirty<T> {
    #[must_use = "The modified value works after flushed into register"]
    pub fn write<Field>(mut self, value: Field::ValueType) -> Self
    where
        Field: bits::field::Field<T> + bits::field::FieldWriter<T>,
        Self: Sized,
    {
        Field::write(&mut self.raw_buffer, value);
        self
    }
    pub fn flush(mut self) -> Clean<T> {
        T::flush(&mut self.raw_buffer);
        Clean {
            raw_buffer: self.raw_buffer,
        }
    }
}

/// 基本的原始锁，没有实现 Drop 自动解锁功能，也不存储任何数据，只有基本 lock 和 unlock 功能。
pub struct ArchMutex {
    flag: AtomicBool,
}

impl ArchMutex {
    pub const fn new() -> Self {
        Self {
            flag: AtomicBool::new(true),
        }
    }
    #[no_mangle]
    pub fn lock(&mut self) -> bool {
        self.flag
            .compare_exchange(true, false, Relaxed, Relaxed)
            .is_ok()
    }
    pub fn unlock(&mut self) -> bool {
        self.flag
            .compare_exchange(false, true, Relaxed, Relaxed)
            .is_ok()
    }
}

#[cfg(test)]
mod test {
    use crate::ArchMutex;

    #[test]
    #[no_mangle]
    fn mutex_test() {
        let mut mutex = ArchMutex::new();
        assert_eq!(true, mutex.lock());
        assert_eq!(false, mutex.lock());
        assert_eq!(true, mutex.unlock());
        assert_eq!(false, mutex.unlock());
    }
}
