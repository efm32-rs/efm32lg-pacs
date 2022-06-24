#[doc = "Register `ECCPARITY` reader"]
pub struct R(crate::R<ECCPARITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCPARITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCPARITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCPARITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECCPARITY` reader - ECC Parity Data"]
pub type ECCPARITY_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC Parity Data"]
    #[inline(always)]
    pub fn eccparity(&self) -> ECCPARITY_R {
        ECCPARITY_R::new(self.bits)
    }
}
#[doc = "ECC Parity register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccparity](index.html) module"]
pub struct ECCPARITY_SPEC;
impl crate::RegisterSpec for ECCPARITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccparity::R](R) reader structure"]
impl crate::Readable for ECCPARITY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECCPARITY to value 0"]
impl crate::Resettable for ECCPARITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
