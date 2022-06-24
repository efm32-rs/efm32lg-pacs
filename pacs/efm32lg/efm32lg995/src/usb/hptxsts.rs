#[doc = "Register `HPTXSTS` reader"]
pub struct R(crate::R<HPTXSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPTXSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPTXSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPTXSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PTXFSPCAVAIL` reader - Periodic Transmit Data FIFO Space Available"]
pub type PTXFSPCAVAIL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PTXQSPCAVAIL` reader - Periodic Transmit Request Queue Space Available"]
pub type PTXQSPCAVAIL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PTXQTOP` reader - Top of the Periodic Transmit Request Queue"]
pub type PTXQTOP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline(always)]
    pub fn ptxfspcavail(&self) -> PTXFSPCAVAIL_R {
        PTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn ptxqspcavail(&self) -> PTXQSPCAVAIL_R {
        PTXQSPCAVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the Periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Host Periodic Transmit FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptxsts](index.html) module"]
pub struct HPTXSTS_SPEC;
impl crate::RegisterSpec for HPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hptxsts::R](R) reader structure"]
impl crate::Readable for HPTXSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HPTXSTS to value 0x0008_0200"]
impl crate::Resettable for HPTXSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0200
    }
}
