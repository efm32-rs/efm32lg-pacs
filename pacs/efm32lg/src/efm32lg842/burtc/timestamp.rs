#[doc = "Register `TIMESTAMP` reader"]
pub struct R(crate::R<TIMESTAMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMESTAMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMESTAMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMESTAMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMESTAMP` reader - Backup mode timestamp."]
pub type TIMESTAMP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Backup mode timestamp."]
    #[inline(always)]
    pub fn timestamp(&self) -> TIMESTAMP_R {
        TIMESTAMP_R::new(self.bits)
    }
}
#[doc = "Backup mode timestamp\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timestamp](index.html) module"]
pub struct TIMESTAMP_SPEC;
impl crate::RegisterSpec for TIMESTAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timestamp::R](R) reader structure"]
impl crate::Readable for TIMESTAMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMESTAMP to value 0"]
impl crate::Resettable for TIMESTAMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
