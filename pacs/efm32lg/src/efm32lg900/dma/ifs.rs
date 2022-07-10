#[doc = "Register `IFS` writer"]
pub struct W(crate::W<IFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS_SPEC>;
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
impl From<crate::W<IFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0DONE` writer - DMA Channel 0 Complete Interrupt Flag Set"]
pub type CH0DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `CH1DONE` writer - DMA Channel 1 Complete Interrupt Flag Set"]
pub type CH1DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `CH2DONE` writer - DMA Channel 2 Complete Interrupt Flag Set"]
pub type CH2DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `CH3DONE` writer - DMA Channel 3 Complete Interrupt Flag Set"]
pub type CH3DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `CH4DONE` writer - DMA Channel 4 Complete Interrupt Flag Set"]
pub type CH4DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
#[doc = "Field `CH5DONE` writer - DMA Channel 5 Complete Interrupt Flag Set"]
pub type CH5DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 5>;
#[doc = "Field `CH6DONE` writer - DMA Channel 6 Complete Interrupt Flag Set"]
pub type CH6DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 6>;
#[doc = "Field `CH7DONE` writer - DMA Channel 7 Complete Interrupt Flag Set"]
pub type CH7DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 7>;
#[doc = "Field `CH8DONE` writer - DMA Channel 8 Complete Interrupt Flag Set"]
pub type CH8DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 8>;
#[doc = "Field `CH9DONE` writer - DMA Channel 9 Complete Interrupt Flag Set"]
pub type CH9DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 9>;
#[doc = "Field `CH10DONE` writer - DMA Channel 10 Complete Interrupt Flag Set"]
pub type CH10DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 10>;
#[doc = "Field `CH11DONE` writer - DMA Channel 11 Complete Interrupt Flag Set"]
pub type CH11DONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 11>;
#[doc = "Field `ERR` writer - DMA Error Interrupt Flag Set"]
pub type ERR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 31>;
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch0done(&mut self) -> CH0DONE_W {
        CH0DONE_W::new(self)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch1done(&mut self) -> CH1DONE_W {
        CH1DONE_W::new(self)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch2done(&mut self) -> CH2DONE_W {
        CH2DONE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch3done(&mut self) -> CH3DONE_W {
        CH3DONE_W::new(self)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch4done(&mut self) -> CH4DONE_W {
        CH4DONE_W::new(self)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch5done(&mut self) -> CH5DONE_W {
        CH5DONE_W::new(self)
    }
    #[doc = "Bit 6 - DMA Channel 6 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch6done(&mut self) -> CH6DONE_W {
        CH6DONE_W::new(self)
    }
    #[doc = "Bit 7 - DMA Channel 7 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch7done(&mut self) -> CH7DONE_W {
        CH7DONE_W::new(self)
    }
    #[doc = "Bit 8 - DMA Channel 8 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch8done(&mut self) -> CH8DONE_W {
        CH8DONE_W::new(self)
    }
    #[doc = "Bit 9 - DMA Channel 9 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch9done(&mut self) -> CH9DONE_W {
        CH9DONE_W::new(self)
    }
    #[doc = "Bit 10 - DMA Channel 10 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch10done(&mut self) -> CH10DONE_W {
        CH10DONE_W::new(self)
    }
    #[doc = "Bit 11 - DMA Channel 11 Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn ch11done(&mut self) -> CH11DONE_W {
        CH11DONE_W::new(self)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Set"]
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
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
