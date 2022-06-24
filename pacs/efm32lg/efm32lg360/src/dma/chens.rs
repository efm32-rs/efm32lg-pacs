#[doc = "Register `CHENS` writer"]
pub struct W(crate::W<CHENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHENS_SPEC>;
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
impl From<crate::W<CHENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0ENS` writer - Channel 0 Enable Set"]
pub type CH0ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 0>;
#[doc = "Field `CH1ENS` writer - Channel 1 Enable Set"]
pub type CH1ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 1>;
#[doc = "Field `CH2ENS` writer - Channel 2 Enable Set"]
pub type CH2ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 2>;
#[doc = "Field `CH3ENS` writer - Channel 3 Enable Set"]
pub type CH3ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 3>;
#[doc = "Field `CH4ENS` writer - Channel 4 Enable Set"]
pub type CH4ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 4>;
#[doc = "Field `CH5ENS` writer - Channel 5 Enable Set"]
pub type CH5ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 5>;
#[doc = "Field `CH6ENS` writer - Channel 6 Enable Set"]
pub type CH6ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 6>;
#[doc = "Field `CH7ENS` writer - Channel 7 Enable Set"]
pub type CH7ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 7>;
#[doc = "Field `CH8ENS` writer - Channel 8 Enable Set"]
pub type CH8ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 8>;
#[doc = "Field `CH9ENS` writer - Channel 9 Enable Set"]
pub type CH9ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 9>;
#[doc = "Field `CH10ENS` writer - Channel 10 Enable Set"]
pub type CH10ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 10>;
#[doc = "Field `CH11ENS` writer - Channel 11 Enable Set"]
pub type CH11ENS_W<'a> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, 11>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable Set"]
    #[inline(always)]
    pub fn ch0ens(&mut self) -> CH0ENS_W {
        CH0ENS_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Enable Set"]
    #[inline(always)]
    pub fn ch1ens(&mut self) -> CH1ENS_W {
        CH1ENS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Enable Set"]
    #[inline(always)]
    pub fn ch2ens(&mut self) -> CH2ENS_W {
        CH2ENS_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Enable Set"]
    #[inline(always)]
    pub fn ch3ens(&mut self) -> CH3ENS_W {
        CH3ENS_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Enable Set"]
    #[inline(always)]
    pub fn ch4ens(&mut self) -> CH4ENS_W {
        CH4ENS_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Enable Set"]
    #[inline(always)]
    pub fn ch5ens(&mut self) -> CH5ENS_W {
        CH5ENS_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Enable Set"]
    #[inline(always)]
    pub fn ch6ens(&mut self) -> CH6ENS_W {
        CH6ENS_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Enable Set"]
    #[inline(always)]
    pub fn ch7ens(&mut self) -> CH7ENS_W {
        CH7ENS_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Enable Set"]
    #[inline(always)]
    pub fn ch8ens(&mut self) -> CH8ENS_W {
        CH8ENS_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Enable Set"]
    #[inline(always)]
    pub fn ch9ens(&mut self) -> CH9ENS_W {
        CH9ENS_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Enable Set"]
    #[inline(always)]
    pub fn ch10ens(&mut self) -> CH10ENS_W {
        CH10ENS_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Enable Set"]
    #[inline(always)]
    pub fn ch11ens(&mut self) -> CH11ENS_W {
        CH11ENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chens](index.html) module"]
pub struct CHENS_SPEC;
impl crate::RegisterSpec for CHENS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chens::W](W) writer structure"]
impl crate::Writable for CHENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHENS to value 0"]
impl crate::Resettable for CHENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
