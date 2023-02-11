#[doc = "Register `CHENC` writer"]
pub struct W(crate::W<CHENC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHENC_SPEC>;
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
impl From<crate::W<CHENC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHENC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0ENC` writer - Channel 0 Enable Clear"]
pub type CH0ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
#[doc = "Field `CH1ENC` writer - Channel 1 Enable Clear"]
pub type CH1ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
#[doc = "Field `CH2ENC` writer - Channel 2 Enable Clear"]
pub type CH2ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
#[doc = "Field `CH3ENC` writer - Channel 3 Enable Clear"]
pub type CH3ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
#[doc = "Field `CH4ENC` writer - Channel 4 Enable Clear"]
pub type CH4ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
#[doc = "Field `CH5ENC` writer - Channel 5 Enable Clear"]
pub type CH5ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
#[doc = "Field `CH6ENC` writer - Channel 6 Enable Clear"]
pub type CH6ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
#[doc = "Field `CH7ENC` writer - Channel 7 Enable Clear"]
pub type CH7ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
#[doc = "Field `CH8ENC` writer - Channel 8 Enable Clear"]
pub type CH8ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
#[doc = "Field `CH9ENC` writer - Channel 9 Enable Clear"]
pub type CH9ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
#[doc = "Field `CH10ENC` writer - Channel 10 Enable Clear"]
pub type CH10ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
#[doc = "Field `CH11ENC` writer - Channel 11 Enable Clear"]
pub type CH11ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0enc(&mut self) -> CH0ENC_W<0> {
        CH0ENC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1enc(&mut self) -> CH1ENC_W<1> {
        CH1ENC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2enc(&mut self) -> CH2ENC_W<2> {
        CH2ENC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3enc(&mut self) -> CH3ENC_W<3> {
        CH3ENC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4enc(&mut self) -> CH4ENC_W<4> {
        CH4ENC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5enc(&mut self) -> CH5ENC_W<5> {
        CH5ENC_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch6enc(&mut self) -> CH6ENC_W<6> {
        CH6ENC_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch7enc(&mut self) -> CH7ENC_W<7> {
        CH7ENC_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch8enc(&mut self) -> CH8ENC_W<8> {
        CH8ENC_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch9enc(&mut self) -> CH9ENC_W<9> {
        CH9ENC_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch10enc(&mut self) -> CH10ENC_W<10> {
        CH10ENC_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch11enc(&mut self) -> CH11ENC_W<11> {
        CH11ENC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chenc](index.html) module"]
pub struct CHENC_SPEC;
impl crate::RegisterSpec for CHENC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chenc::W](W) writer structure"]
impl crate::Writable for CHENC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHENC to value 0"]
impl crate::Resettable for CHENC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
