#[doc = "Register `CHPRIS` writer"]
pub struct W(crate::W<CHPRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPRIS_SPEC>;
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
impl From<crate::W<CHPRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0PRIS` writer - Channel 0 High Priority Set"]
pub type CH0PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 0>;
#[doc = "Field `CH1PRIS` writer - Channel 1 High Priority Set"]
pub type CH1PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 1>;
#[doc = "Field `CH2PRIS` writer - Channel 2 High Priority Set"]
pub type CH2PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 2>;
#[doc = "Field `CH3PRIS` writer - Channel 3 High Priority Set"]
pub type CH3PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 3>;
#[doc = "Field `CH4PRIS` writer - Channel 4 High Priority Set"]
pub type CH4PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 4>;
#[doc = "Field `CH5PRIS` writer - Channel 5 High Priority Set"]
pub type CH5PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 5>;
#[doc = "Field `CH6PRIS` writer - Channel 6 High Priority Set"]
pub type CH6PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 6>;
#[doc = "Field `CH7PRIS` writer - Channel 7 High Priority Set"]
pub type CH7PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 7>;
#[doc = "Field `CH8PRIS` writer - Channel 8 High Priority Set"]
pub type CH8PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 8>;
#[doc = "Field `CH9PRIS` writer - Channel 9 High Priority Set"]
pub type CH9PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 9>;
#[doc = "Field `CH10PRIS` writer - Channel 10 High Priority Set"]
pub type CH10PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 10>;
#[doc = "Field `CH11PRIS` writer - Channel 11 High Priority Set"]
pub type CH11PRIS_W<'a> = crate::BitWriter<'a, u32, CHPRIS_SPEC, bool, 11>;
impl W {
    #[doc = "Bit 0 - Channel 0 High Priority Set"]
    #[inline(always)]
    pub fn ch0pris(&mut self) -> CH0PRIS_W {
        CH0PRIS_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 High Priority Set"]
    #[inline(always)]
    pub fn ch1pris(&mut self) -> CH1PRIS_W {
        CH1PRIS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 High Priority Set"]
    #[inline(always)]
    pub fn ch2pris(&mut self) -> CH2PRIS_W {
        CH2PRIS_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 High Priority Set"]
    #[inline(always)]
    pub fn ch3pris(&mut self) -> CH3PRIS_W {
        CH3PRIS_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 High Priority Set"]
    #[inline(always)]
    pub fn ch4pris(&mut self) -> CH4PRIS_W {
        CH4PRIS_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 High Priority Set"]
    #[inline(always)]
    pub fn ch5pris(&mut self) -> CH5PRIS_W {
        CH5PRIS_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 High Priority Set"]
    #[inline(always)]
    pub fn ch6pris(&mut self) -> CH6PRIS_W {
        CH6PRIS_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 High Priority Set"]
    #[inline(always)]
    pub fn ch7pris(&mut self) -> CH7PRIS_W {
        CH7PRIS_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 High Priority Set"]
    #[inline(always)]
    pub fn ch8pris(&mut self) -> CH8PRIS_W {
        CH8PRIS_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 High Priority Set"]
    #[inline(always)]
    pub fn ch9pris(&mut self) -> CH9PRIS_W {
        CH9PRIS_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 High Priority Set"]
    #[inline(always)]
    pub fn ch10pris(&mut self) -> CH10PRIS_W {
        CH10PRIS_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 High Priority Set"]
    #[inline(always)]
    pub fn ch11pris(&mut self) -> CH11PRIS_W {
        CH11PRIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Priority Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpris](index.html) module"]
pub struct CHPRIS_SPEC;
impl crate::RegisterSpec for CHPRIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chpris::W](W) writer structure"]
impl crate::Writable for CHPRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHPRIS to value 0"]
impl crate::Resettable for CHPRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
