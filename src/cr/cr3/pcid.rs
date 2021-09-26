use register::{RegisterBufferFlush, RegisterBufferReader, RegisterBufferWriter};

use crate::Clean;

use super::{fields, Cr3Buffer};

pub struct Cr3BufferPcid {
    pub(super) buffer: Cr3Buffer,
}
impl RegisterBufferFlush for Cr3BufferPcid {
    fn flush(&mut self) {
        self.buffer.flush()
    }
}

impl crate::Dirty<Cr3BufferPcid> {
    /// 用于写 PCID 的辅助函数
    pub fn set_pcid(mut self, id: u16) -> Self {
        {
            self.raw_buffer.buffer.write::<fields::PCID>(id)
        };
        self
    }
}

impl Clean<Cr3BufferPcid> {
    /// 用于读 pcid 的辅助函数
    pub fn pcid(&self) -> u16 {
        self.raw_buffer.buffer.read::<fields::PCID>()
    }
}

impl_reg_buffer_trait! {
    Cr3BufferPcid;
}
