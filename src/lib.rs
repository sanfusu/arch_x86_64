#![feature(asm)]
#![no_std]

macro_rules! impl_buffer_trait {
    ($($(#[$Attr:meta])? $Buffer:ident);+ $(;)?) => {
        $(
            $(#[$Attr])?
            impl $Buffer {
                pub fn read<T: bits::field::Field<Self> + bits::field::FieldReader<Self>>(&self) -> T::ValueType {
                    T::read(self)
                }
                pub fn output<T: bits::field::Field<Self> + bits::field::FieldReader<Self>>(
                    &self,
                    out: &mut T::ValueType,
                ) -> &Self {
                    *out = T::read(self);
                    self
                }

                #[must_use = "The modified value works after flushed into register"]
                pub fn write<T>(&mut self, value: T::ValueType) -> &mut Self
                where
                    T: bits::field::Field<Self> + bits::field::FieldWriter<Self>,
                {
                    T::write(self, value);
                    self
                }
                #[must_use = "The modified value works after flushed into register"]
                pub fn revert<T>(&mut self) -> &mut Self
                where
                    T: bits::field::Field<Self> + bits::field::FieldWriter<Self>,
                {
                    T::revert(self);
                    self
                }
            }
        )+
    };
}

pub mod cpuid;
pub mod cr;
pub mod descriptor;
pub mod msr;
