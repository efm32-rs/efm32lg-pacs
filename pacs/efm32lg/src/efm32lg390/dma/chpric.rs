#[doc = "Register `CHPRIC` writer"]
pub struct W(crate::W<CHPRIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPRIC_SPEC>;
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
impl From<crate::W<CHPRIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPRIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0PRIC` writer - Channel 0 High Priority Clear"]
pub type CH0PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH1PRIC` writer - Channel 1 High Priority Clear"]
pub type CH1PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH2PRIC` writer - Channel 2 High Priority Clear"]
pub type CH2PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH3PRIC` writer - Channel 3 High Priority Clear"]
pub type CH3PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH4PRIC` writer - Channel 4 High Priority Clear"]
pub type CH4PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH5PRIC` writer - Channel 5 High Priority Clear"]
pub type CH5PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH6PRIC` writer - Channel 6 High Priority Clear"]
pub type CH6PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH7PRIC` writer - Channel 7 High Priority Clear"]
pub type CH7PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH8PRIC` writer - Channel 8 High Priority Clear"]
pub type CH8PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH9PRIC` writer - Channel 9 High Priority Clear"]
pub type CH9PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH10PRIC` writer - Channel 10 High Priority Clear"]
pub type CH10PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH11PRIC` writer - Channel 11 High Priority Clear"]
pub type CH11PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pric(&mut self) -> CH0PRIC_W<0> {
        CH0PRIC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pric(&mut self) -> CH1PRIC_W<1> {
        CH1PRIC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pric(&mut self) -> CH2PRIC_W<2> {
        CH2PRIC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pric(&mut self) -> CH3PRIC_W<3> {
        CH3PRIC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pric(&mut self) -> CH4PRIC_W<4> {
        CH4PRIC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pric(&mut self) -> CH5PRIC_W<5> {
        CH5PRIC_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch6pric(&mut self) -> CH6PRIC_W<6> {
        CH6PRIC_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch7pric(&mut self) -> CH7PRIC_W<7> {
        CH7PRIC_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch8pric(&mut self) -> CH8PRIC_W<8> {
        CH8PRIC_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch9pric(&mut self) -> CH9PRIC_W<9> {
        CH9PRIC_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch10pric(&mut self) -> CH10PRIC_W<10> {
        CH10PRIC_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch11pric(&mut self) -> CH11PRIC_W<11> {
        CH11PRIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Priority Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpric](index.html) module"]
pub struct CHPRIC_SPEC;
impl crate::RegisterSpec for CHPRIC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chpric::W](W) writer structure"]
impl crate::Writable for CHPRIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHPRIC to value 0"]
impl crate::Resettable for CHPRIC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
