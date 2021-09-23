use core::fmt::Display;

#[derive(Debug)]
pub struct StdFeature {
    pub(crate) ecx: u32,
    pub(crate) edx: u32,
}

impl Display for StdFeature {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            fmt,
            "MSR: {msr}\nSSE: {sse3}",
            msr = self.support_msr(),
            sse3 = self.support_sse3()
        )
    }
}
impl_reg_buffer_trait!(StdFeature);

plain_field! {
    StdFeature {
        pub support_msr:    fields::MSR,
        pub support_fpu:    fields::FPU,
        pub support_sse3:   fields::SSE3,
        pub support_pcid:   fields::PCID
    }
}

pub mod fields {
    use super::StdFeature;

    bits::fields_ex! {
       StdFeature [ecx] {
           pub SSE3 [00, ro, bool],
           pub PCID [17, ro, bool]
       }
       StdFeature [edx] {
           pub FPU [0, ro, bool],
           pub MSR [5, ro, bool],
       }
    }
}
