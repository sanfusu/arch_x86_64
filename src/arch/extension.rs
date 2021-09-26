use crate::{
    cpuid::{feature::StdFeature, Cpuid},
    cr::cr4::{Cr4, Cr4Buffer},
    msr::efer::Efer,
    Clean,
};

/// 唯一占据空间的是 std_feature：8 个这字节，其余字段不占用空间。
///
/// 构建本结构体的最直接方法：调用 `let arch_extension = Arch::init()?.extension()?`
pub struct ArchExtension {
    pub std_feature: StdFeature,
    pub cpuid: Cpuid,
    pub efer: Efer,
    pub cr4: Cr4,
}
impl ArchExtension {
    pub fn pcid_extension(&self) -> Option<PcidExtension> {
        if self.efer.buffer()?.long_mode_activated() && self.std_feature.support_pcid() {
            Some(PcidExtension {
                cr4_buffer: self.cr4.buffer()?,
            })
        } else {
            None
        }
    }
}

pub struct PcidExtension {
    cr4_buffer: Clean<Cr4Buffer>,
}
impl PcidExtension {
    pub fn disable(self) {
        unsafe {
            // disable 和 enable 的安全性均由 ArchExtension::pcid_extension 函数保证
            self.cr4_buffer.asume_dirty().disable_pcid_uncheck().flush();
        }
    }
    pub fn enable(self) {
        unsafe {
            self.cr4_buffer.asume_dirty().enable_pcid_uncheck().flush();
        }
    }
    pub fn enabled(&self) -> bool {
        unsafe { self.cr4_buffer.pcid_enabled() }
    }
    pub fn disabled(&self) -> bool {
        unsafe { !self.cr4_buffer.pcid_enabled() }
    }
}
