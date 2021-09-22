use core::fmt::Display;

#[derive(Debug)]
pub struct Feature {
    pub(crate) ecx: u32,
    pub(crate) edx: u32,
}
impl Display for Feature {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            fmt,
            "MSR: {msr}\nSSE: {sse3}",
            msr = self.msr(),
            sse3 = self.sse3()
        )
    }
}
impl_reg_buffer_trait!(Feature);

plain_field! {
    Feature {
        pub msr:    fields::MSR,
        pub fpu:    fields::FPU,
        pub sse3:   fields::SSE3
    }
}

pub mod fields {
    use super::Feature;

    bits::fields_ex! {
       Feature [ecx] {
           pub SSE3 [0, ro, bool]
       }
       Feature [edx] {
           pub FPU [0, ro, bool],
           pub MSR [5, ro, bool],
       }
    }
}
