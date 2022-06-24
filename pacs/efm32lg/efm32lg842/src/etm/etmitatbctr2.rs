#[doc = "Register `ETMITATBCTR2` reader"]
pub struct R(crate::R<ETMITATBCTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETMITATBCTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETMITATBCTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETMITATBCTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ATREADY` reader - ATREADY Input Value"]
pub type ATREADY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - ATREADY Input Value"]
    #[inline(always)]
    pub fn atready(&self) -> ATREADY_R {
        ATREADY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "ETM Integration Test ATB Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmitatbctr2](index.html) module"]
pub struct ETMITATBCTR2_SPEC;
impl crate::RegisterSpec for ETMITATBCTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etmitatbctr2::R](R) reader structure"]
impl crate::Readable for ETMITATBCTR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETMITATBCTR2 to value 0x01"]
impl crate::Resettable for ETMITATBCTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
