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
#[doc = "Field `TXC` reader - TX Complete Interrupt Enable"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - TX Complete Interrupt Enable"]
pub type TXC_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `TXBL` reader - TX Buffer Level Interrupt Enable"]
pub type TXBL_R = crate::BitReader<bool>;
#[doc = "Field `TXBL` writer - TX Buffer Level Interrupt Enable"]
pub type TXBL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `RXDATAV` reader - RX Data Valid Interrupt Enable"]
pub type RXDATAV_R = crate::BitReader<bool>;
#[doc = "Field `RXDATAV` writer - RX Data Valid Interrupt Enable"]
pub type RXDATAV_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `RXOF` reader - RX Overflow Interrupt Enable"]
pub type RXOF_R = crate::BitReader<bool>;
#[doc = "Field `RXOF` writer - RX Overflow Interrupt Enable"]
pub type RXOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `RXUF` reader - RX Underflow Interrupt Enable"]
pub type RXUF_R = crate::BitReader<bool>;
#[doc = "Field `RXUF` writer - RX Underflow Interrupt Enable"]
pub type RXUF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `TXOF` reader - TX Overflow Interrupt Enable"]
pub type TXOF_R = crate::BitReader<bool>;
#[doc = "Field `TXOF` writer - TX Overflow Interrupt Enable"]
pub type TXOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
#[doc = "Field `PERR` reader - Parity Error Interrupt Enable"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `PERR` writer - Parity Error Interrupt Enable"]
pub type PERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 6>;
#[doc = "Field `FERR` reader - Framing Error Interrupt Enable"]
pub type FERR_R = crate::BitReader<bool>;
#[doc = "Field `FERR` writer - Framing Error Interrupt Enable"]
pub type FERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 7>;
#[doc = "Field `MPAF` reader - Multi-Processor Address Frame Interrupt Enable"]
pub type MPAF_R = crate::BitReader<bool>;
#[doc = "Field `MPAF` writer - Multi-Processor Address Frame Interrupt Enable"]
pub type MPAF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
#[doc = "Field `STARTF` reader - Start Frame Interrupt Enable"]
pub type STARTF_R = crate::BitReader<bool>;
#[doc = "Field `STARTF` writer - Start Frame Interrupt Enable"]
pub type STARTF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 9>;
#[doc = "Field `SIGF` reader - Signal Frame Interrupt Enable"]
pub type SIGF_R = crate::BitReader<bool>;
#[doc = "Field `SIGF` writer - Signal Frame Interrupt Enable"]
pub type SIGF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - TX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Processor Address Frame Interrupt Enable"]
    #[inline(always)]
    pub fn mpaf(&self) -> MPAF_R {
        MPAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start Frame Interrupt Enable"]
    #[inline(always)]
    pub fn startf(&self) -> STARTF_R {
        STARTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Signal Frame Interrupt Enable"]
    #[inline(always)]
    pub fn sigf(&self) -> SIGF_R {
        SIGF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W::new(self)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&mut self) -> TXBL_W {
        TXBL_W::new(self)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&mut self) -> RXDATAV_W {
        RXDATAV_W::new(self)
    }
    #[doc = "Bit 3 - RX Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RXOF_W {
        RXOF_W::new(self)
    }
    #[doc = "Bit 4 - RX Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W::new(self)
    }
    #[doc = "Bit 5 - TX Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W::new(self)
    }
    #[doc = "Bit 6 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W::new(self)
    }
    #[doc = "Bit 7 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FERR_W {
        FERR_W::new(self)
    }
    #[doc = "Bit 8 - Multi-Processor Address Frame Interrupt Enable"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MPAF_W {
        MPAF_W::new(self)
    }
    #[doc = "Bit 9 - Start Frame Interrupt Enable"]
    #[inline(always)]
    pub fn startf(&mut self) -> STARTF_W {
        STARTF_W::new(self)
    }
    #[doc = "Bit 10 - Signal Frame Interrupt Enable"]
    #[inline(always)]
    pub fn sigf(&mut self) -> SIGF_W {
        SIGF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
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
