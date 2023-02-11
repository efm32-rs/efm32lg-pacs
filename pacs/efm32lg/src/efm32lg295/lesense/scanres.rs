#[doc = "Register `SCANRES` reader"]
pub struct R(crate::R<SCANRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SCANRES` reader - Scan results"]
pub type SCANRES_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Scan results"]
    #[inline(always)]
    pub fn scanres(&self) -> SCANRES_R {
        SCANRES_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Scan result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanres](index.html) module"]
pub struct SCANRES_SPEC;
impl crate::RegisterSpec for SCANRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scanres::R](R) reader structure"]
impl crate::Readable for SCANRES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCANRES to value 0"]
impl crate::Resettable for SCANRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
