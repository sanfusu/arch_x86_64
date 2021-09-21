pub struct Feature {
    ecx: u32,
    edx: u32,
}

impl_reg_buffer_trait!(Feature);

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
