#[doc = "Register `SCANDATAP` reader"]
pub struct R(crate::R<SCANDATAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANDATAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANDATAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANDATAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATAP` reader - Scan Conversion Result Data Peek"]
pub type DATAP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DATAP_R {
        DATAP_R::new(self.bits)
    }
}
#[doc = "Scan Sequence Result Data Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scandatap](index.html) module"]
pub struct SCANDATAP_SPEC;
impl crate::RegisterSpec for SCANDATAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scandatap::R](R) reader structure"]
impl crate::Readable for SCANDATAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCANDATAP to value 0"]
impl crate::Resettable for SCANDATAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
