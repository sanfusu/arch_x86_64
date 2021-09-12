#![feature(asm)]
#![no_std]

macro_rules! impl_buffer_trait {
    ($($(#[$Attr:meta])? $Buffer:ident);+ $(;)?) => {
        $(
            $(#[$Attr])?
            impl bits::field::BufferReader for $Buffer {
                fn read<T: bits::field::Field<Self> + bits::field::FieldReader<Self>>(&self) -> T::ValueType {
                    T::read(self)
                }

                fn output<T: bits::field::Field<Self> + bits::field::FieldReader<Self>>(
                    &self,
                    out: &mut T::ValueType,
                ) -> &Self {
                    *out = T::read(self);
                    self
                }
            }
            $(#[$Attr])?
            impl bits::field::BufferWriter for $Buffer {
                #[must_use = "The modified value works after flushed into register"]
                fn write<T>(&mut self, value: T::ValueType) -> &mut Self
                where
                    T: bits::field::Field<Self> + bits::field::FieldWriter<Self>,
                {
                    T::write(self, value);
                    self
                }

                #[must_use = "The modified value works after flushed into register"]
                fn revert<T>(&mut self) -> &mut Self
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
pub mod msr;
