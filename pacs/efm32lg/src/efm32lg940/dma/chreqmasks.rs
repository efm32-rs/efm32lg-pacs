#[doc = "Register `CHREQMASKS` writer"]
pub struct W(crate::W<CHREQMASKS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHREQMASKS_SPEC>;
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
impl From<crate::W<CHREQMASKS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHREQMASKS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0REQMASKS` writer - Channel 0 Request Mask Set"]
pub type CH0REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 0>;
#[doc = "Field `CH1REQMASKS` writer - Channel 1 Request Mask Set"]
pub type CH1REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 1>;
#[doc = "Field `CH2REQMASKS` writer - Channel 2 Request Mask Set"]
pub type CH2REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 2>;
#[doc = "Field `CH3REQMASKS` writer - Channel 3 Request Mask Set"]
pub type CH3REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 3>;
#[doc = "Field `CH4REQMASKS` writer - Channel 4 Request Mask Set"]
pub type CH4REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 4>;
#[doc = "Field `CH5REQMASKS` writer - Channel 5 Request Mask Set"]
pub type CH5REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 5>;
#[doc = "Field `CH6REQMASKS` writer - Channel 6 Request Mask Set"]
pub type CH6REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 6>;
#[doc = "Field `CH7REQMASKS` writer - Channel 7 Request Mask Set"]
pub type CH7REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 7>;
#[doc = "Field `CH8REQMASKS` writer - Channel 8 Request Mask Set"]
pub type CH8REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 8>;
#[doc = "Field `CH9REQMASKS` writer - Channel 9 Request Mask Set"]
pub type CH9REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 9>;
#[doc = "Field `CH10REQMASKS` writer - Channel 10 Request Mask Set"]
pub type CH10REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 10>;
#[doc = "Field `CH11REQMASKS` writer - Channel 11 Request Mask Set"]
pub type CH11REQMASKS_W<'a> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, 11>;
impl W {
    #[doc = "Bit 0 - Channel 0 Request Mask Set"]
    #[inline(always)]
    pub fn ch0reqmasks(&mut self) -> CH0REQMASKS_W {
        CH0REQMASKS_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Request Mask Set"]
    #[inline(always)]
    pub fn ch1reqmasks(&mut self) -> CH1REQMASKS_W {
        CH1REQMASKS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Request Mask Set"]
    #[inline(always)]
    pub fn ch2reqmasks(&mut self) -> CH2REQMASKS_W {
        CH2REQMASKS_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Request Mask Set"]
    #[inline(always)]
    pub fn ch3reqmasks(&mut self) -> CH3REQMASKS_W {
        CH3REQMASKS_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Request Mask Set"]
    #[inline(always)]
    pub fn ch4reqmasks(&mut self) -> CH4REQMASKS_W {
        CH4REQMASKS_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Request Mask Set"]
    #[inline(always)]
    pub fn ch5reqmasks(&mut self) -> CH5REQMASKS_W {
        CH5REQMASKS_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Request Mask Set"]
    #[inline(always)]
    pub fn ch6reqmasks(&mut self) -> CH6REQMASKS_W {
        CH6REQMASKS_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Request Mask Set"]
    #[inline(always)]
    pub fn ch7reqmasks(&mut self) -> CH7REQMASKS_W {
        CH7REQMASKS_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Request Mask Set"]
    #[inline(always)]
    pub fn ch8reqmasks(&mut self) -> CH8REQMASKS_W {
        CH8REQMASKS_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Request Mask Set"]
    #[inline(always)]
    pub fn ch9reqmasks(&mut self) -> CH9REQMASKS_W {
        CH9REQMASKS_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Request Mask Set"]
    #[inline(always)]
    pub fn ch10reqmasks(&mut self) -> CH10REQMASKS_W {
        CH10REQMASKS_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Request Mask Set"]
    #[inline(always)]
    pub fn ch11reqmasks(&mut self) -> CH11REQMASKS_W {
        CH11REQMASKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Request Mask Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chreqmasks](index.html) module"]
pub struct CHREQMASKS_SPEC;
impl crate::RegisterSpec for CHREQMASKS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chreqmasks::W](W) writer structure"]
impl crate::Writable for CHREQMASKS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHREQMASKS to value 0"]
impl crate::Resettable for CHREQMASKS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
