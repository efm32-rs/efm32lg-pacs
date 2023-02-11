#[doc = "Register `DIEP0_INT` reader"]
pub struct R(crate::R<DIEP0_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP0_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP0_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP0_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP0_INT` writer"]
pub struct W(crate::W<DIEP0_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP0_INT_SPEC>;
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
impl From<crate::W<DIEP0_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP0_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERCOMPL` reader - Transfer Completed Interrupt"]
pub type XFERCOMPL_R = crate::BitReader<bool>;
#[doc = "Field `XFERCOMPL` writer - Transfer Completed Interrupt"]
pub type XFERCOMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0_INT_SPEC, bool, O>;
#[doc = "Field `EPDISBLD` reader - Endpoint Disabled Interrupt"]
pub type EPDISBLD_R = crate::BitReader<bool>;
#[doc = "Field `EPDISBLD` writer - Endpoint Disabled Interrupt"]
pub type EPDISBLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0_INT_SPEC, bool, O>;
#[doc = "Field `AHBERR` reader - AHB Error"]
pub type AHBERR_R = crate::BitReader<bool>;
#[doc = "Field `AHBERR` writer - AHB Error"]
pub type AHBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0_INT_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` reader - Timeout Condition"]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` writer - Timeout Condition"]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0_INT_SPEC, bool, O>;
#[doc = "Field `INTKNTXFEMP` reader - IN Token Received When TxFIFO is Empty"]
pub type INTKNTXFEMP_R = crate::BitReader<bool>;
#[doc = "Field `INTKNTXFEMP` writer - IN Token Received When TxFIFO is Empty"]
pub type INTKNTXFEMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0_INT_SPEC, bool, O>;
#[doc = "Field `INEPNAKEFF` reader - IN Endpoint NAK Effective"]
pub type INEPNAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `INEPNAKEFF` writer - IN Endpoint NAK Effective"]
pub type INEPNAKEFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0_INT_SPEC, bool, O>;
#[doc = "Field `TXFEMP` reader - Transmit FIFO Empty"]
pub type TXFEMP_R = crate::BitReader<bool>;
#[doc = "Field `PKTDRPSTS` reader - Packet Drop Status"]
pub type PKTDRPSTS_R = crate::BitReader<bool>;
#[doc = "Field `PKTDRPSTS` writer - Packet Drop Status"]
pub type PKTDRPSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0_INT_SPEC, bool, O>;
#[doc = "Field `BBLEERR` reader - NAK Interrupt"]
pub type BBLEERR_R = crate::BitReader<bool>;
#[doc = "Field `BBLEERR` writer - NAK Interrupt"]
pub type BBLEERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0_INT_SPEC, bool, O>;
#[doc = "Field `NAKINTRPT` reader - NAK Interrupt"]
pub type NAKINTRPT_R = crate::BitReader<bool>;
#[doc = "Field `NAKINTRPT` writer - NAK Interrupt"]
pub type NAKINTRPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0_INT_SPEC, bool, O>;
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
    #[must_use]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W<0> {
        XFERCOMPL_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld(&mut self) -> EPDISBLD_W<1> {
        EPDISBLD_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<2> {
        AHBERR_W::new(self)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<3> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfemp(&mut self) -> INTKNTXFEMP_W<4> {
        INTKNTXFEMP_W::new(self)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    #[must_use]
    pub fn inepnakeff(&mut self) -> INEPNAKEFF_W<6> {
        INEPNAKEFF_W::new(self)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<11> {
        PKTDRPSTS_W::new(self)
    }
    #[doc = "Bit 12 - NAK Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerr(&mut self) -> BBLEERR_W<12> {
        BBLEERR_W::new(self)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt(&mut self) -> NAKINTRPT_W<13> {
        NAKINTRPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN Endpoint x+1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0_int](index.html) module"]
pub struct DIEP0_INT_SPEC;
impl crate::RegisterSpec for DIEP0_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep0_int::R](R) reader structure"]
impl crate::Readable for DIEP0_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep0_int::W](W) writer structure"]
impl crate::Writable for DIEP0_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP0_INT to value 0x80"]
impl crate::Resettable for DIEP0_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
