#[doc = "Register `COMBDATA` writer"]
pub struct W(crate::W<COMBDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMBDATA_SPEC>;
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
impl From<crate::W<COMBDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMBDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0DATA` writer - Channel 0 Data"]
pub type CH0DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMBDATA_SPEC, u16, u16, 12, O>;
#[doc = "Field `CH1DATA` writer - Channel 1 Data"]
pub type CH1DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMBDATA_SPEC, u16, u16, 12, O>;
impl W {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    #[must_use]
    pub fn ch0data(&mut self) -> CH0DATA_W<0> {
        CH0DATA_W::new(self)
    }
    #[doc = "Bits 16:27 - Channel 1 Data"]
    #[inline(always)]
    #[must_use]
    pub fn ch1data(&mut self) -> CH1DATA_W<16> {
        CH1DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combined Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [combdata](index.html) module"]
pub struct COMBDATA_SPEC;
impl crate::RegisterSpec for COMBDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [combdata::W](W) writer structure"]
impl crate::Writable for COMBDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMBDATA to value 0"]
impl crate::Resettable for COMBDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
