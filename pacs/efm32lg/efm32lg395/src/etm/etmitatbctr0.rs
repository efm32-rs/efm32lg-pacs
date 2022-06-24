#[doc = "Register `ETMITATBCTR0` writer"]
pub struct W(crate::W<ETMITATBCTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETMITATBCTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ETMITATBCTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETMITATBCTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATVALID` writer - ATVALID Output Value"]
pub type ATVALID_W<'a> = crate::BitWriter<'a, u32, ETMITATBCTR0_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 0 - ATVALID Output Value"]
    #[inline(always)]
    pub fn atvalid(&mut self) -> ATVALID_W {
        ATVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Integration Test ATB Control 0 Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmitatbctr0](index.html) module"]
pub struct ETMITATBCTR0_SPEC;
impl crate::RegisterSpec for ETMITATBCTR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [etmitatbctr0::W](W) writer structure"]
impl crate::Writable for ETMITATBCTR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETMITATBCTR0 to value 0"]
impl crate::Resettable for ETMITATBCTR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
