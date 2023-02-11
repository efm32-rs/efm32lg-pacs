#[doc = "Register `TFTSTATUS` reader"]
pub struct R(crate::R<TFTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HCNT` reader - Horizontal Count"]
pub type HCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VCNT` reader - Vertical Count"]
pub type VCNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Horizontal Count"]
    #[inline(always)]
    pub fn hcnt(&self) -> HCNT_R {
        HCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Vertical Count"]
    #[inline(always)]
    pub fn vcnt(&self) -> VCNT_R {
        VCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "TFT Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftstatus](index.html) module"]
pub struct TFTSTATUS_SPEC;
impl crate::RegisterSpec for TFTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tftstatus::R](R) reader structure"]
impl crate::Readable for TFTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TFTSTATUS to value 0"]
impl crate::Resettable for TFTSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
