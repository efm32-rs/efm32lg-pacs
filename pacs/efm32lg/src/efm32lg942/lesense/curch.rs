#[doc = "Register `CURCH` reader"]
pub struct R(crate::R<CURCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURCH` reader - Shows the index of the current channel"]
pub type CURCH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Shows the index of the current channel"]
    #[inline(always)]
    pub fn curch(&self) -> CURCH_R {
        CURCH_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Current channel index\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [curch](index.html) module"]
pub struct CURCH_SPEC;
impl crate::RegisterSpec for CURCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [curch::R](R) reader structure"]
impl crate::Readable for CURCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CURCH to value 0"]
impl crate::Resettable for CURCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
