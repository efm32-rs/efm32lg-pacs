#[doc = "Register `CHREQMASKC` writer"]
pub struct W(crate::W<CHREQMASKC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHREQMASKC_SPEC>;
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
impl From<crate::W<CHREQMASKC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHREQMASKC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0REQMASKC` writer - Channel 0 Request Mask Clear"]
pub type CH0REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
#[doc = "Field `CH1REQMASKC` writer - Channel 1 Request Mask Clear"]
pub type CH1REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
#[doc = "Field `CH2REQMASKC` writer - Channel 2 Request Mask Clear"]
pub type CH2REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
#[doc = "Field `CH3REQMASKC` writer - Channel 3 Request Mask Clear"]
pub type CH3REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
#[doc = "Field `CH4REQMASKC` writer - Channel 4 Request Mask Clear"]
pub type CH4REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
#[doc = "Field `CH5REQMASKC` writer - Channel 5 Request Mask Clear"]
pub type CH5REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
#[doc = "Field `CH6REQMASKC` writer - Channel 6 Request Mask Clear"]
pub type CH6REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
#[doc = "Field `CH7REQMASKC` writer - Channel 7 Request Mask Clear"]
pub type CH7REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
#[doc = "Field `CH8REQMASKC` writer - Channel 8 Request Mask Clear"]
pub type CH8REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
#[doc = "Field `CH9REQMASKC` writer - Channel 9 Request Mask Clear"]
pub type CH9REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
#[doc = "Field `CH10REQMASKC` writer - Channel 10 Request Mask Clear"]
pub type CH10REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
#[doc = "Field `CH11REQMASKC` writer - Channel 11 Request Mask Clear"]
pub type CH11REQMASKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0reqmaskc(&mut self) -> CH0REQMASKC_W<0> {
        CH0REQMASKC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1reqmaskc(&mut self) -> CH1REQMASKC_W<1> {
        CH1REQMASKC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2reqmaskc(&mut self) -> CH2REQMASKC_W<2> {
        CH2REQMASKC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3reqmaskc(&mut self) -> CH3REQMASKC_W<3> {
        CH3REQMASKC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4reqmaskc(&mut self) -> CH4REQMASKC_W<4> {
        CH4REQMASKC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5reqmaskc(&mut self) -> CH5REQMASKC_W<5> {
        CH5REQMASKC_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch6reqmaskc(&mut self) -> CH6REQMASKC_W<6> {
        CH6REQMASKC_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch7reqmaskc(&mut self) -> CH7REQMASKC_W<7> {
        CH7REQMASKC_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch8reqmaskc(&mut self) -> CH8REQMASKC_W<8> {
        CH8REQMASKC_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch9reqmaskc(&mut self) -> CH9REQMASKC_W<9> {
        CH9REQMASKC_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch10reqmaskc(&mut self) -> CH10REQMASKC_W<10> {
        CH10REQMASKC_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch11reqmaskc(&mut self) -> CH11REQMASKC_W<11> {
        CH11REQMASKC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Request Mask Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chreqmaskc](index.html) module"]
pub struct CHREQMASKC_SPEC;
impl crate::RegisterSpec for CHREQMASKC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chreqmaskc::W](W) writer structure"]
impl crate::Writable for CHREQMASKC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHREQMASKC to value 0"]
impl crate::Resettable for CHREQMASKC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
