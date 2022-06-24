#[doc = "Register `DIEP2_INT` reader"]
pub struct R(crate::R<DIEP2_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP2_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP2_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP2_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP2_INT` writer"]
pub struct W(crate::W<DIEP2_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP2_INT_SPEC>;
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
impl From<crate::W<DIEP2_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP2_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERCOMPL` reader - Transfer Completed Interrupt"]
pub type XFERCOMPL_R = crate::BitReader<bool>;
#[doc = "Field `XFERCOMPL` writer - Transfer Completed Interrupt"]
pub type XFERCOMPL_W<'a> = crate::BitWriter<'a, u32, DIEP2_INT_SPEC, bool, 0>;
#[doc = "Field `EPDISBLD` reader - Endpoint Disabled Interrupt"]
pub type EPDISBLD_R = crate::BitReader<bool>;
#[doc = "Field `EPDISBLD` writer - Endpoint Disabled Interrupt"]
pub type EPDISBLD_W<'a> = crate::BitWriter<'a, u32, DIEP2_INT_SPEC, bool, 1>;
#[doc = "Field `AHBERR` reader - AHB Error"]
pub type AHBERR_R = crate::BitReader<bool>;
#[doc = "Field `AHBERR` writer - AHB Error"]
pub type AHBERR_W<'a> = crate::BitWriter<'a, u32, DIEP2_INT_SPEC, bool, 2>;
#[doc = "Field `TIMEOUT` reader - Timeout Condition"]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` writer - Timeout Condition"]
pub type TIMEOUT_W<'a> = crate::BitWriter<'a, u32, DIEP2_INT_SPEC, bool, 3>;
#[doc = "Field `INTKNTXFEMP` reader - IN Token Received When TxFIFO is Empty"]
pub type INTKNTXFEMP_R = crate::BitReader<bool>;
#[doc = "Field `INTKNTXFEMP` writer - IN Token Received When TxFIFO is Empty"]
pub type INTKNTXFEMP_W<'a> = crate::BitWriter<'a, u32, DIEP2_INT_SPEC, bool, 4>;
#[doc = "Field `INEPNAKEFF` reader - IN Endpoint NAK Effective"]
pub type INEPNAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `INEPNAKEFF` writer - IN Endpoint NAK Effective"]
pub type INEPNAKEFF_W<'a> = crate::BitWriter<'a, u32, DIEP2_INT_SPEC, bool, 6>;
#[doc = "Field `TXFEMP` reader - Transmit FIFO Empty"]
pub type TXFEMP_R = crate::BitReader<bool>;
#[doc = "Field `PKTDRPSTS` reader - Packet Drop Status"]
pub type PKTDRPSTS_R = crate::BitReader<bool>;
#[doc = "Field `PKTDRPSTS` writer - Packet Drop Status"]
pub type PKTDRPSTS_W<'a> = crate::BitWriter<'a, u32, DIEP2_INT_SPEC, bool, 11>;
#[doc = "Field `BBLEERR` reader - NAK Interrupt"]
pub type BBLEERR_R = crate::BitReader<bool>;
#[doc = "Field `BBLEERR` writer - NAK Interrupt"]
pub type BBLEERR_W<'a> = crate::BitWriter<'a, u32, DIEP2_INT_SPEC, bool, 12>;
#[doc = "Field `NAKINTRPT` reader - NAK Interrupt"]
pub type NAKINTRPT_R = crate::BitReader<bool>;
#[doc = "Field `NAKINTRPT` writer - NAK Interrupt"]
pub type NAKINTRPT_W<'a> = crate::BitWriter<'a, u32, DIEP2_INT_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EPDISBLD_R {
        EPDISBLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    pub fn intkntxfemp(&self) -> INTKNTXFEMP_R {
        INTKNTXFEMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    pub fn inepnakeff(&self) -> INEPNAKEFF_R {
        INEPNAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty"]
    #[inline(always)]
    pub fn txfemp(&self) -> TXFEMP_R {
        TXFEMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NAK Interrupt"]
    #[inline(always)]
    pub fn bbleerr(&self) -> BBLEERR_R {
        BBLEERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NAKINTRPT_R {
        NAKINTRPT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W {
        XFERCOMPL_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&mut self) -> EPDISBLD_W {
        EPDISBLD_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W {
        AHBERR_W::new(self)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    pub fn intkntxfemp(&mut self) -> INTKNTXFEMP_W {
        INTKNTXFEMP_W::new(self)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    pub fn inepnakeff(&mut self) -> INEPNAKEFF_W {
        INEPNAKEFF_W::new(self)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W {
        PKTDRPSTS_W::new(self)
    }
    #[doc = "Bit 12 - NAK Interrupt"]
    #[inline(always)]
    pub fn bbleerr(&mut self) -> BBLEERR_W {
        BBLEERR_W::new(self)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&mut self) -> NAKINTRPT_W {
        NAKINTRPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN Endpoint x+1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2_int](index.html) module"]
pub struct DIEP2_INT_SPEC;
impl crate::RegisterSpec for DIEP2_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep2_int::R](R) reader structure"]
impl crate::Readable for DIEP2_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep2_int::W](W) writer structure"]
impl crate::Writable for DIEP2_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEP2_INT to value 0x80"]
impl crate::Resettable for DIEP2_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
