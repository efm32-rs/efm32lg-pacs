#[doc = "Register `ETMIDR2` reader"]
pub struct R(crate::R<ETMIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFE` reader - RFE Transfer Order"]
pub type RFE_R = crate::BitReader<bool>;
#[doc = "Field `SWP` reader - SWP Transfer Order"]
pub type SWP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RFE Transfer Order"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWP Transfer Order"]
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "ETM ID Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmidr2](index.html) module"]
pub struct ETMIDR2_SPEC;
impl crate::RegisterSpec for ETMIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmidr2::R](R) reader structure"]
impl crate::Readable for ETMIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMIDR2 to value 0"]
impl crate::Resettable for ETMIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
