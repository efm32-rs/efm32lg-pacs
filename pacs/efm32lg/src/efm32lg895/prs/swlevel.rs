#[doc = "Register `SWLEVEL` reader"]
pub struct R(crate::R<SWLEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWLEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWLEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWLEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWLEVEL` writer"]
pub struct W(crate::W<SWLEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWLEVEL_SPEC>;
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
impl From<crate::W<SWLEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWLEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0LEVEL` reader - Channel 0 Software Level"]
pub type CH0LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH0LEVEL` writer - Channel 0 Software Level"]
pub type CH0LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 0>;
#[doc = "Field `CH1LEVEL` reader - Channel 1 Software Level"]
pub type CH1LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH1LEVEL` writer - Channel 1 Software Level"]
pub type CH1LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 1>;
#[doc = "Field `CH2LEVEL` reader - Channel 2 Software Level"]
pub type CH2LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH2LEVEL` writer - Channel 2 Software Level"]
pub type CH2LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 2>;
#[doc = "Field `CH3LEVEL` reader - Channel 3 Software Level"]
pub type CH3LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH3LEVEL` writer - Channel 3 Software Level"]
pub type CH3LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 3>;
#[doc = "Field `CH4LEVEL` reader - Channel 4 Software Level"]
pub type CH4LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH4LEVEL` writer - Channel 4 Software Level"]
pub type CH4LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 4>;
#[doc = "Field `CH5LEVEL` reader - Channel 5 Software Level"]
pub type CH5LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH5LEVEL` writer - Channel 5 Software Level"]
pub type CH5LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 5>;
#[doc = "Field `CH6LEVEL` reader - Channel 6 Software Level"]
pub type CH6LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH6LEVEL` writer - Channel 6 Software Level"]
pub type CH6LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 6>;
#[doc = "Field `CH7LEVEL` reader - Channel 7 Software Level"]
pub type CH7LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH7LEVEL` writer - Channel 7 Software Level"]
pub type CH7LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 7>;
#[doc = "Field `CH8LEVEL` reader - Channel 8 Software Level"]
pub type CH8LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH8LEVEL` writer - Channel 8 Software Level"]
pub type CH8LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 8>;
#[doc = "Field `CH9LEVEL` reader - Channel 9 Software Level"]
pub type CH9LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH9LEVEL` writer - Channel 9 Software Level"]
pub type CH9LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 9>;
#[doc = "Field `CH10LEVEL` reader - Channel 10 Software Level"]
pub type CH10LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH10LEVEL` writer - Channel 10 Software Level"]
pub type CH10LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 10>;
#[doc = "Field `CH11LEVEL` reader - Channel 11 Software Level"]
pub type CH11LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH11LEVEL` writer - Channel 11 Software Level"]
pub type CH11LEVEL_W<'a> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&self) -> CH0LEVEL_R {
        CH0LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&self) -> CH1LEVEL_R {
        CH1LEVEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&self) -> CH2LEVEL_R {
        CH2LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&self) -> CH3LEVEL_R {
        CH3LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&self) -> CH4LEVEL_R {
        CH4LEVEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&self) -> CH5LEVEL_R {
        CH5LEVEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Level"]
    #[inline(always)]
    pub fn ch6level(&self) -> CH6LEVEL_R {
        CH6LEVEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Level"]
    #[inline(always)]
    pub fn ch7level(&self) -> CH7LEVEL_R {
        CH7LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Software Level"]
    #[inline(always)]
    pub fn ch8level(&self) -> CH8LEVEL_R {
        CH8LEVEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Software Level"]
    #[inline(always)]
    pub fn ch9level(&self) -> CH9LEVEL_R {
        CH9LEVEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Software Level"]
    #[inline(always)]
    pub fn ch10level(&self) -> CH10LEVEL_R {
        CH10LEVEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Software Level"]
    #[inline(always)]
    pub fn ch11level(&self) -> CH11LEVEL_R {
        CH11LEVEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&mut self) -> CH0LEVEL_W {
        CH0LEVEL_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&mut self) -> CH1LEVEL_W {
        CH1LEVEL_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&mut self) -> CH2LEVEL_W {
        CH2LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&mut self) -> CH3LEVEL_W {
        CH3LEVEL_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&mut self) -> CH4LEVEL_W {
        CH4LEVEL_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&mut self) -> CH5LEVEL_W {
        CH5LEVEL_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Software Level"]
    #[inline(always)]
    pub fn ch6level(&mut self) -> CH6LEVEL_W {
        CH6LEVEL_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Software Level"]
    #[inline(always)]
    pub fn ch7level(&mut self) -> CH7LEVEL_W {
        CH7LEVEL_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Software Level"]
    #[inline(always)]
    pub fn ch8level(&mut self) -> CH8LEVEL_W {
        CH8LEVEL_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Software Level"]
    #[inline(always)]
    pub fn ch9level(&mut self) -> CH9LEVEL_W {
        CH9LEVEL_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Software Level"]
    #[inline(always)]
    pub fn ch10level(&mut self) -> CH10LEVEL_W {
        CH10LEVEL_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Software Level"]
    #[inline(always)]
    pub fn ch11level(&mut self) -> CH11LEVEL_W {
        CH11LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swlevel](index.html) module"]
pub struct SWLEVEL_SPEC;
impl crate::RegisterSpec for SWLEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swlevel::R](R) reader structure"]
impl crate::Readable for SWLEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swlevel::W](W) writer structure"]
impl crate::Writable for SWLEVEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWLEVEL to value 0"]
impl crate::Resettable for SWLEVEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
