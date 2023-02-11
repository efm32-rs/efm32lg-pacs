#[doc = "Register `IFC` writer"]
pub struct W(crate::W<IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFC_SPEC>;
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
impl From<crate::W<IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0DONE` writer - DMA Channel 0 Complete Interrupt Flag Clear"]
pub type CH0DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH1DONE` writer - DMA Channel 1 Complete Interrupt Flag Clear"]
pub type CH1DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH2DONE` writer - DMA Channel 2 Complete Interrupt Flag Clear"]
pub type CH2DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH3DONE` writer - DMA Channel 3 Complete Interrupt Flag Clear"]
pub type CH3DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH4DONE` writer - DMA Channel 4 Complete Interrupt Flag Clear"]
pub type CH4DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH5DONE` writer - DMA Channel 5 Complete Interrupt Flag Clear"]
pub type CH5DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH6DONE` writer - DMA Channel 6 Complete Interrupt Flag Clear"]
pub type CH6DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH7DONE` writer - DMA Channel 7 Complete Interrupt Flag Clear"]
pub type CH7DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH8DONE` writer - DMA Channel 8 Complete Interrupt Flag Clear"]
pub type CH8DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH9DONE` writer - DMA Channel 9 Complete Interrupt Flag Clear"]
pub type CH9DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH10DONE` writer - DMA Channel 10 Complete Interrupt Flag Clear"]
pub type CH10DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH11DONE` writer - DMA Channel 11 Complete Interrupt Flag Clear"]
pub type CH11DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `ERR` writer - DMA Error Interrupt Flag Clear"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0done(&mut self) -> CH0DONE_W<0> {
        CH0DONE_W::new(self)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1done(&mut self) -> CH1DONE_W<1> {
        CH1DONE_W::new(self)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2done(&mut self) -> CH2DONE_W<2> {
        CH2DONE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3done(&mut self) -> CH3DONE_W<3> {
        CH3DONE_W::new(self)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4done(&mut self) -> CH4DONE_W<4> {
        CH4DONE_W::new(self)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5done(&mut self) -> CH5DONE_W<5> {
        CH5DONE_W::new(self)
    }
    #[doc = "Bit 6 - DMA Channel 6 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch6done(&mut self) -> CH6DONE_W<6> {
        CH6DONE_W::new(self)
    }
    #[doc = "Bit 7 - DMA Channel 7 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch7done(&mut self) -> CH7DONE_W<7> {
        CH7DONE_W::new(self)
    }
    #[doc = "Bit 8 - DMA Channel 8 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch8done(&mut self) -> CH8DONE_W<8> {
        CH8DONE_W::new(self)
    }
    #[doc = "Bit 9 - DMA Channel 9 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch9done(&mut self) -> CH9DONE_W<9> {
        CH9DONE_W::new(self)
    }
    #[doc = "Bit 10 - DMA Channel 10 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch10done(&mut self) -> CH10DONE_W<10> {
        CH10DONE_W::new(self)
    }
    #[doc = "Bit 11 - DMA Channel 11 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch11done(&mut self) -> CH11DONE_W<11> {
        CH11DONE_W::new(self)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<31> {
        ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifc::W](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
