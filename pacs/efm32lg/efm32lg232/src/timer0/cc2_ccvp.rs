#[doc = "Register `CC2_CCVP` reader"]
pub struct R(crate::R<CC2_CCVP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC2_CCVP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC2_CCVP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC2_CCVP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CCVP` reader - CC Channel Value Peek"]
pub type CCVP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Peek"]
    #[inline(always)]
    pub fn ccvp(&self) -> CCVP_R {
        CCVP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CC Channel Value Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc2_ccvp](index.html) module"]
pub struct CC2_CCVP_SPEC;
impl crate::RegisterSpec for CC2_CCVP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc2_ccvp::R](R) reader structure"]
impl crate::Readable for CC2_CCVP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CC2_CCVP to value 0"]
impl crate::Resettable for CC2_CCVP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
