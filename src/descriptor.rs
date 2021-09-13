#[repr(C)]
pub struct Descriptor {
    low: u32,
    high: u32,
}

impl_buffer_trait!(Descriptor);

pub enum TssDescriptor {
    Available16bit,
    Available32bit,
    Busy32bit,
}
pub enum SystemDescriptor {
    LDT,
    TSS,
    GATE,
}
pub enum UserDescriptor {
    Code,
    Data,
}
pub enum DescriptorType {
    User(UserDescriptor),
    System(SystemDescriptor),
}

pub mod fields {
    use bits::{
        field::{BufferReader, BufferWriter, Field, FieldReader, FieldWriter},
        BitsOps, IntoBits,
    };

    use super::Descriptor;

    pub struct BaseAddress;
    impl Field<Descriptor> for BaseAddress {
        type ValueType = usize;
    }
    impl FieldReader<Descriptor> for BaseAddress {
        fn read(buffer: &Descriptor) -> Self::ValueType {
            (buffer.read::<BaseAddress1>() as usize)
                | ((buffer.read::<BaseAddress2>() as usize) << 16)
                | ((buffer.read::<BaseAddress3>() as usize) << 24)
        }
    }
    impl FieldWriter<Descriptor> for BaseAddress {
        fn write(buffer: &mut Descriptor, value: Self::ValueType) {
            {
                buffer
                    .write::<BaseAddress1>(value.bits(0..=15).read() as u16)
                    .write::<BaseAddress2>(value.bits(16..=23).read() as u8)
                    .write::<BaseAddress3>(value.bits(24..=31).read() as u8)
            };
        }

        fn revert(buffer: &mut Descriptor) {
            {
                buffer
                    .revert::<BaseAddress1>()
                    .revert::<BaseAddress2>()
                    .revert::<BaseAddress3>()
            };
        }
    }

    pub struct SegLimit;
    impl Field<Descriptor> for SegLimit {
        type ValueType = u32;
    }
    impl FieldReader<Descriptor> for SegLimit {
        fn read(buffer: &Descriptor) -> Self::ValueType {
            (buffer.read::<SegLimit1>() as u32) | ((buffer.read::<SegLimit2>() as u32) << 16)
        }
    }
    impl FieldWriter<Descriptor> for SegLimit {
        fn write(buffer: &mut Descriptor, value: Self::ValueType) {
            {
                buffer
                    .write::<SegLimit1>(value.bits(0..=15).read() as u16)
                    .write::<SegLimit2>(value.bits(16..=19).read() as u8)
            };
        }

        fn revert(buffer: &mut Descriptor) {
            {
                buffer.revert::<SegLimit1>().revert::<SegLimit2>()
            };
        }
    }
    bits::fields_ex! {
        Descriptor [low] {
            SegLimit1       [00..=15, rw, u16],
            BaseAddress1    [16..=31, rw, u16]
        }
        Descriptor [high] {
            BaseAddress2    [00..=07, rw, u8],
            Type            [08..=11, rw, u8],
            S               [12, rw, bool],
            DPL             [13..=14, rw, u8],
            P               [15, rw, bool],
            SegLimit2       [16..=19, rw, u8],
            AVL             [20, rw, bool],
            L               [21, rw, bool],
            DB              [22, rw, bool],
            G               [23, rw, bool],
            BaseAddress3    [24..=31, rw, u8],
        }
    }
}
