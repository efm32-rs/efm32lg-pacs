#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0DONE` reader - DMA Channel 0 Complete Interrupt Enable"]
pub type CH0DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH0DONE` writer - DMA Channel 0 Complete Interrupt Enable"]
pub type CH0DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `CH1DONE` reader - DMA Channel 1 Complete Interrupt Enable"]
pub type CH1DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH1DONE` writer - DMA Channel 1 Complete Interrupt Enable"]
pub type CH1DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `CH2DONE` reader - DMA Channel 2 Complete Interrupt Enable"]
pub type CH2DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH2DONE` writer - DMA Channel 2 Complete Interrupt Enable"]
pub type CH2DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `CH3DONE` reader - DMA Channel 3 Complete Interrupt Enable"]
pub type CH3DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH3DONE` writer - DMA Channel 3 Complete Interrupt Enable"]
pub type CH3DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `CH4DONE` reader - DMA Channel 4 Complete Interrupt Enable"]
pub type CH4DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH4DONE` writer - DMA Channel 4 Complete Interrupt Enable"]
pub type CH4DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `CH5DONE` reader - DMA Channel 5 Complete Interrupt Enable"]
pub type CH5DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH5DONE` writer - DMA Channel 5 Complete Interrupt Enable"]
pub type CH5DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
#[doc = "Field `CH6DONE` reader - DMA Channel 6 Complete Interrupt Enable"]
pub type CH6DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH6DONE` writer - DMA Channel 6 Complete Interrupt Enable"]
pub type CH6DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 6>;
#[doc = "Field `CH7DONE` reader - DMA Channel 7 Complete Interrupt Enable"]
pub type CH7DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH7DONE` writer - DMA Channel 7 Complete Interrupt Enable"]
pub type CH7DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 7>;
#[doc = "Field `CH8DONE` reader - DMA Channel 8 Complete Interrupt Enable"]
pub type CH8DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH8DONE` writer - DMA Channel 8 Complete Interrupt Enable"]
pub type CH8DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
#[doc = "Field `CH9DONE` reader - DMA Channel 9 Complete Interrupt Enable"]
pub type CH9DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH9DONE` writer - DMA Channel 9 Complete Interrupt Enable"]
pub type CH9DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 9>;
#[doc = "Field `CH10DONE` reader - DMA Channel 10 Complete Interrupt Enable"]
pub type CH10DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH10DONE` writer - DMA Channel 10 Complete Interrupt Enable"]
pub type CH10DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 10>;
#[doc = "Field `CH11DONE` reader - DMA Channel 11 Complete Interrupt Enable"]
pub type CH11DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH11DONE` writer - DMA Channel 11 Complete Interrupt Enable"]
pub type CH11DONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 11>;
#[doc = "Field `ERR` reader - DMA Error Interrupt Flag Enable"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - DMA Error Interrupt Flag Enable"]
pub type ERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch0done(&self) -> CH0DONE_R {
        CH0DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch1done(&self) -> CH1DONE_R {
        CH1DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch2done(&self) -> CH2DONE_R {
        CH2DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch3done(&self) -> CH3DONE_R {
        CH3DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch4done(&self) -> CH4DONE_R {
        CH4DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch5done(&self) -> CH5DONE_R {
        CH5DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Channel 6 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch6done(&self) -> CH6DONE_R {
        CH6DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Channel 7 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch7done(&self) -> CH7DONE_R {
        CH7DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Channel 8 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch8done(&self) -> CH8DONE_R {
        CH8DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA Channel 9 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch9done(&self) -> CH9DONE_R {
        CH9DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA Channel 10 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch10done(&self) -> CH10DONE_R {
        CH10DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Channel 11 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch11done(&self) -> CH11DONE_R {
        CH11DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Enable"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch0done(&mut self) -> CH0DONE_W {
        CH0DONE_W::new(self)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch1done(&mut self) -> CH1DONE_W {
        CH1DONE_W::new(self)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch2done(&mut self) -> CH2DONE_W {
        CH2DONE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch3done(&mut self) -> CH3DONE_W {
        CH3DONE_W::new(self)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch4done(&mut self) -> CH4DONE_W {
        CH4DONE_W::new(self)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch5done(&mut self) -> CH5DONE_W {
        CH5DONE_W::new(self)
    }
    #[doc = "Bit 6 - DMA Channel 6 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch6done(&mut self) -> CH6DONE_W {
        CH6DONE_W::new(self)
    }
    #[doc = "Bit 7 - DMA Channel 7 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch7done(&mut self) -> CH7DONE_W {
        CH7DONE_W::new(self)
    }
    #[doc = "Bit 8 - DMA Channel 8 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch8done(&mut self) -> CH8DONE_W {
        CH8DONE_W::new(self)
    }
    #[doc = "Bit 9 - DMA Channel 9 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch9done(&mut self) -> CH9DONE_W {
        CH9DONE_W::new(self)
    }
    #[doc = "Bit 10 - DMA Channel 10 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch10done(&mut self) -> CH10DONE_W {
        CH10DONE_W::new(self)
    }
    #[doc = "Bit 11 - DMA Channel 11 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch11done(&mut self) -> CH11DONE_W {
        CH11DONE_W::new(self)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Enable"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
