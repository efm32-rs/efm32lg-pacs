#[doc = "Register `SINGLEDATAP` reader"]
pub struct R(crate::R<SINGLEDATAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLEDATAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLEDATAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLEDATAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATAP` reader - Single Conversion Result Data Peek"]
pub type DATAP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Single Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DATAP_R {
        DATAP_R::new(self.bits)
    }
}
#[doc = "Single Conversion Result Data Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singledatap](index.html) module"]
pub struct SINGLEDATAP_SPEC;
impl crate::RegisterSpec for SINGLEDATAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singledatap::R](R) reader structure"]
impl crate::Readable for SINGLEDATAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SINGLEDATAP to value 0"]
impl crate::Resettable for SINGLEDATAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
