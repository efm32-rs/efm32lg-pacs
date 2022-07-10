#[doc = "Register `BUFDATA` reader"]
pub struct R(crate::R<BUFDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUFDATA` reader - Result data"]
pub type BUFDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Result data"]
    #[inline(always)]
    pub fn bufdata(&self) -> BUFDATA_R {
        BUFDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Result buffer data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufdata](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct BUFDATA_SPEC;
impl crate::RegisterSpec for BUFDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bufdata::R](R) reader structure"]
impl crate::Readable for BUFDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUFDATA to value 0"]
impl crate::Resettable for BUFDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
