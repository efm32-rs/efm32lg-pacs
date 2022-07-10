#[doc = "Register `CHALTC` writer"]
pub struct W(crate::W<CHALTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHALTC_SPEC>;
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
impl From<crate::W<CHALTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHALTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0ALTC` writer - Channel 0 Alternate Clear"]
pub type CH0ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 0>;
#[doc = "Field `CH1ALTC` writer - Channel 1 Alternate Clear"]
pub type CH1ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 1>;
#[doc = "Field `CH2ALTC` writer - Channel 2 Alternate Clear"]
pub type CH2ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 2>;
#[doc = "Field `CH3ALTC` writer - Channel 3 Alternate Clear"]
pub type CH3ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 3>;
#[doc = "Field `CH4ALTC` writer - Channel 4 Alternate Clear"]
pub type CH4ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 4>;
#[doc = "Field `CH5ALTC` writer - Channel 5 Alternate Clear"]
pub type CH5ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 5>;
#[doc = "Field `CH6ALTC` writer - Channel 6 Alternate Clear"]
pub type CH6ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 6>;
#[doc = "Field `CH7ALTC` writer - Channel 7 Alternate Clear"]
pub type CH7ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 7>;
#[doc = "Field `CH8ALTC` writer - Channel 8 Alternate Clear"]
pub type CH8ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 8>;
#[doc = "Field `CH9ALTC` writer - Channel 9 Alternate Clear"]
pub type CH9ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 9>;
#[doc = "Field `CH10ALTC` writer - Channel 10 Alternate Clear"]
pub type CH10ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 10>;
#[doc = "Field `CH11ALTC` writer - Channel 11 Alternate Clear"]
pub type CH11ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 11>;
impl W {
    #[doc = "Bit 0 - Channel 0 Alternate Clear"]
    #[inline(always)]
    pub fn ch0altc(&mut self) -> CH0ALTC_W {
        CH0ALTC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Alternate Clear"]
    #[inline(always)]
    pub fn ch1altc(&mut self) -> CH1ALTC_W {
        CH1ALTC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Alternate Clear"]
    #[inline(always)]
    pub fn ch2altc(&mut self) -> CH2ALTC_W {
        CH2ALTC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Alternate Clear"]
    #[inline(always)]
    pub fn ch3altc(&mut self) -> CH3ALTC_W {
        CH3ALTC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Alternate Clear"]
    #[inline(always)]
    pub fn ch4altc(&mut self) -> CH4ALTC_W {
        CH4ALTC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Alternate Clear"]
    #[inline(always)]
    pub fn ch5altc(&mut self) -> CH5ALTC_W {
        CH5ALTC_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Alternate Clear"]
    #[inline(always)]
    pub fn ch6altc(&mut self) -> CH6ALTC_W {
        CH6ALTC_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Alternate Clear"]
    #[inline(always)]
    pub fn ch7altc(&mut self) -> CH7ALTC_W {
        CH7ALTC_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Alternate Clear"]
    #[inline(always)]
    pub fn ch8altc(&mut self) -> CH8ALTC_W {
        CH8ALTC_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Alternate Clear"]
    #[inline(always)]
    pub fn ch9altc(&mut self) -> CH9ALTC_W {
        CH9ALTC_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Alternate Clear"]
    #[inline(always)]
    pub fn ch10altc(&mut self) -> CH10ALTC_W {
        CH10ALTC_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Alternate Clear"]
    #[inline(always)]
    pub fn ch11altc(&mut self) -> CH11ALTC_W {
        CH11ALTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Alternate Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chaltc](index.html) module"]
pub struct CHALTC_SPEC;
impl crate::RegisterSpec for CHALTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chaltc::W](W) writer structure"]
impl crate::Writable for CHALTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHALTC to value 0"]
impl crate::Resettable for CHALTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
