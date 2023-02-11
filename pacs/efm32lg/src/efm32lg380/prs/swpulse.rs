#[doc = "Register `SWPULSE` writer"]
pub struct W(crate::W<SWPULSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWPULSE_SPEC>;
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
impl From<crate::W<SWPULSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWPULSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0PULSE` writer - Channel 0 Pulse Generation"]
pub type CH0PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
#[doc = "Field `CH1PULSE` writer - Channel 1 Pulse Generation"]
pub type CH1PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
#[doc = "Field `CH2PULSE` writer - Channel 2 Pulse Generation"]
pub type CH2PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
#[doc = "Field `CH3PULSE` writer - Channel 3 Pulse Generation"]
pub type CH3PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
#[doc = "Field `CH4PULSE` writer - Channel 4 Pulse Generation"]
pub type CH4PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
#[doc = "Field `CH5PULSE` writer - Channel 5 Pulse Generation"]
pub type CH5PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
#[doc = "Field `CH6PULSE` writer - Channel 6 Pulse Generation"]
pub type CH6PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
#[doc = "Field `CH7PULSE` writer - Channel 7 Pulse Generation"]
pub type CH7PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
#[doc = "Field `CH8PULSE` writer - Channel 8 Pulse Generation"]
pub type CH8PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
#[doc = "Field `CH9PULSE` writer - Channel 9 Pulse Generation"]
pub type CH9PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
#[doc = "Field `CH10PULSE` writer - Channel 10 Pulse Generation"]
pub type CH10PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
#[doc = "Field `CH11PULSE` writer - Channel 11 Pulse Generation"]
pub type CH11PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPULSE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pulse(&mut self) -> CH0PULSE_W<0> {
        CH0PULSE_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pulse(&mut self) -> CH1PULSE_W<1> {
        CH1PULSE_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pulse(&mut self) -> CH2PULSE_W<2> {
        CH2PULSE_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pulse(&mut self) -> CH3PULSE_W<3> {
        CH3PULSE_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pulse(&mut self) -> CH4PULSE_W<4> {
        CH4PULSE_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pulse(&mut self) -> CH5PULSE_W<5> {
        CH5PULSE_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch6pulse(&mut self) -> CH6PULSE_W<6> {
        CH6PULSE_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch7pulse(&mut self) -> CH7PULSE_W<7> {
        CH7PULSE_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch8pulse(&mut self) -> CH8PULSE_W<8> {
        CH8PULSE_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch9pulse(&mut self) -> CH9PULSE_W<9> {
        CH9PULSE_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch10pulse(&mut self) -> CH10PULSE_W<10> {
        CH10PULSE_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch11pulse(&mut self) -> CH11PULSE_W<11> {
        CH11PULSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Pulse Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpulse](index.html) module"]
pub struct SWPULSE_SPEC;
impl crate::RegisterSpec for SWPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swpulse::W](W) writer structure"]
impl crate::Writable for SWPULSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWPULSE to value 0"]
impl crate::Resettable for SWPULSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
