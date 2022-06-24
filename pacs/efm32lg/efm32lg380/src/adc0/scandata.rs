#[doc = "Register `SCANDATA` reader"]
pub struct R(crate::R<SCANDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Scan Conversion Result Data"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Conversion Result Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Scan Conversion Result Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scandata](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct SCANDATA_SPEC;
impl crate::RegisterSpec for SCANDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scandata::R](R) reader structure"]
impl crate::Readable for SCANDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCANDATA to value 0"]
impl crate::Resettable for SCANDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
