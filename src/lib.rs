#![feature(asm)]
#![no_std]

use register::RegisterBufferReader;

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

/// 只读缓冲区，只能从寄存器直接生成，而不能从原始可读写缓冲区转换。
/// 主要用做部分函数的输入参数，这些函数利用缓冲区来判断寄存器中的内容，而非直接从寄存器中读取，
///
/// 使用 Ro<T> 可以防止寄存器的内容被修改。
/// rust 自身语法只能将入参限制为 mut 变量，而无法限制其为非 mut 变量。
/// 也就是说 mut var，也可以作为非 mut 变量传递给函数。
///
/// ```
/// fn unmut(_: &u16) {}
/// fn test() {
///     let mut v = 0;
///     unmut(&v);
///     v = 2;
///     unmut(&v); // 我们希望 v 在传入之前不会被修改，本函数调用之后的修改情况则不管。
/// }
/// ```
pub struct Ro<T: RegisterBufferReader> {
    pub(crate) rw_buffer: T,
}
impl<T: RegisterBufferReader> Ro<T> {
    pub fn read<Field: bits::field::Field<T> + bits::field::FieldReader<T>>(
        &self,
    ) -> Field::ValueType {
        Field::read(&self.rw_buffer)
    }
    pub fn output<Field: bits::field::Field<T> + bits::field::FieldReader<T>>(
        &self,
        out: &mut Field::ValueType,
    ) -> &Self {
        *out = Field::read(&self.rw_buffer);
        self
    }
    /// 转换为可读写缓冲区
    pub fn into_rw(self) -> T {
        self.rw_buffer
    }
}
