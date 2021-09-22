use core::marker::PhantomData;

use super::selector::Selector;

pub struct Cs {
    phantom: PhantomData<u16>,
}
pub struct CsBuffer {
    pub selector: Selector,
}
impl Cs {
    #[no_mangle]
    pub fn buffer() -> CsBuffer {
        let mut ret = CsBuffer {
            selector: Selector { data: 0 },
        };
        unsafe { asm!("mov {0:x}, cs", out(reg) ret.selector.data) }
        ret
    }
}

#[cfg(test)]
mod test {
    use std::println;

    use bits::field::BufferReader;

    use crate::mem::segment::selector::fields;

    use super::Cs;

    #[test]
    fn cs_test() {
        let cs = Cs::buffer();
        println!("{}", cs.selector.read::<fields::RPL>());
    }
}
